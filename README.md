# autosplash

A simple, configurable, automated wallpaper changer that pulls wallpapers from [Unsplash Source](https://source.unsplash.com/). You can customize search queries, wallpaper resolution, and the time interval for wallpaper changes.

## Compatibility
This software uses the [wallpaper](https://docs.rs/wallpaper/2.0.1/wallpaper/) crate, which in theory should be compatible with Windows, macOS and most of the popular Linux DEs. Currently only been tested on KDE Plasma 5 and Windows 10, the latter of which isn't working as of yet.

## To-do
- [ ] Move utility functions to separate file
- [ ] Reload config file before each wallpaper change
- [ ] Switch to `clap` for command line arguments
- [ ] Add setting for maximum wallpaper cache size
- [ ] Add argument to clear cache
- [ ] Windows 10/macOS compatibility
- [ ] Better error handling

## License
This project is licensed under the GNU GPLv3 license. Please see the LICENSE file for details.