extern crate ini;
extern crate rand;
extern crate reqwest;
extern crate chrono;
extern crate chan;
extern crate termion;

use std::env;
use ini::Ini;
use rand::seq::SliceRandom;
use std::fs::File;
use chrono::prelude::{DateTime, Local};
use chan::chan_select;
use termion::{style, color};

/// TODO
/// - Re-load config on each change
/// - Include --clear flag to clear wallpaper cache
/// - Use clap crate for flags
/// - Handle more errors
/// - Windows compatibility
fn main() {

    println!("\n{}autosplash{} v0.1.0", style::Bold, style::Reset);

    // check if --once flag passed
    let mut once: bool = false;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && args[1] == "--once" {
        once = true;
    }

    // read config file
    let conf = Ini::load_from_file("autosplash.conf").expect("Configuration file missing");
    let settings = conf.section(Some("Settings".to_owned())).expect("Configuration file improperly formatted");

    // get categories
    let categories: Vec<&str> = settings.get("categories").unwrap().split(", ").collect();

    // validate daily setting
    let daily = settings.get("daily").unwrap();
    if !daily.trim().parse::<bool>().is_ok() {
        println!("Config error: daily must be true or false.");
        return;
    }
    let daily: bool = daily.trim().parse::<bool>().unwrap();

    // validate res_width
    let res_width = settings.get("res_width").unwrap();
    if !res_width.trim().parse::<u16>().is_ok() {
        println!("Config error: res_width must be a valid number.");
    }
    let res_width: u16 = res_width.trim().parse::<u16>().unwrap();

    // validate res_height
    let res_height = settings.get("res_height").unwrap();
    if !res_height.trim().parse::<u16>().is_ok() {
        println!("Config error: res_height must be a valid number.");
    }
    let res_height: u16 = res_height.trim().parse::<u16>().unwrap();

    // validate interval
    let int_amount = settings.get("int_amount").unwrap();
    if !int_amount.trim().parse::<f64>().is_ok() {
        println!("Config error: int_amount must be a valid number.");
    }
    let int_amount: f64 = int_amount.trim().parse::<f64>().unwrap();

    // validate interval unit
    let int_unit = settings.get("int_unit").unwrap();
    if !["seconds", "minutes", "hours", "days"].iter().any(|x| x == int_unit) {
        println!("Config error: \"{}\" is not a valid time unit. Valid units: seconds, minutes, hours, days", int_unit);
        return;
    }

    // take unit and multiply given interval by its representation in ms
    let int_ms = int_amount * (match int_unit.as_ref() {
        "seconds" => 1000,
        "minutes" => 1000 * 60,
        "hours" => 1000 * 60 * 60,
        "days" => 1000 * 60 * 60 * 24,
        _ => 0
    } as f64);

    // check if interval exceeds u32 limit
    if int_ms > (u32::max_value() as f64) {
        println!("Config error: interval must be shorter than 49 days");
        return;
    }

    // cast to usize
    let int_ms: u32 = int_ms as u32;

    // Print out config info nicely
    if once {
        print!("Pulling a single random photo from categor");
        if categories.len() == 1 {print!("y ");} else {print!("ies ");}
    } else {
        print!("Pulling random photos from categor");
        if categories.len() == 1 {print!("y ");} else {print!("ies ");}
    }
    for (i, cat) in categories.iter().enumerate() {
        print!("{}", cat);
        if (i+1) < categories.len() {print!(", ");}
    }
    if !once {
        print!(" every ");
        if int_amount != 1.0 {
            println!("{} {}", int_amount, int_unit);
        } else {
            let s_len = int_unit.len() - 1;
            let s = &int_unit[..s_len];
            print!("{}", s);
        }
    }
    println!("\n");

    // set the wallpaper immediately
    change_wallpaper(categories.clone(), res_width, res_height);

    // if --once flag not passed, start the loop
    if !once {
        let tick = chan::tick_ms(int_ms);
        loop {
            chan_select! {
                tick.recv() => change_wallpaper(categories.clone(), res_width, res_height),
            }
        }
    }

}

/// Chooses a random category, requests a wallpaper
/// from Unsplash and sets it.
fn change_wallpaper(categories: Vec<&str>, width: u16, height: u16) {

    // select random category
    let cat = categories.choose(&mut rand::thread_rng()).unwrap();

    // create unsplash request URL
    let unsplash_url = format!("https://source.unsplash.com/{}x{}/?{}", width, height, cat);
    
    // get current date and time to create unique filename
    let local: DateTime<Local> = Local::now();
    let timestamp = local.format("%Y-%m-%d_%H-%M-%S").to_string();

    // create directory (if necessary) and create file object to save wallpaper to
    if !std::fs::metadata("wallpapers").is_ok() {
        std::fs::create_dir("wallpapers").expect("Failed to create wallpapers directory");
    }
    let filename = format!("wallpapers/autosplash_{}.jpg", timestamp);
    let file_path = std::env::current_dir().unwrap().join(filename.as_str());
    let mut file = File::create(&file_path).unwrap();

    // request image from unsplash and save it
    let mut resp = reqwest::get(unsplash_url.as_str()).unwrap();
    std::io::copy(&mut resp, &mut file).expect("Failed to save wallpaper");

    // set the wallpaper from the downloaded file
    wallpaper::set_from_path(&file_path.to_str().unwrap()).expect("Failed to set wallpaper");

}
