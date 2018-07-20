# Raven
A theme manager for linux, currently focusing on i3. Supports multiple different configuration files, and can reload an entire theme on a dime. Fast, easy to use and portable.

## Installation
All you technically require is [cargo](https://github.com/rust-lang/cargo) to be installed.
Run:

`git clone https://github.com/nicohman/raven.git && cd raven`

`cargo build --release`

`sudo cp targets/release/raven /usr/bin/raven`

The following packages are required for their relevant options:

+ [Feh](https://github.com/derf/feh): `wall`

+ [Polybar](https://github.com/jaagr/polybar): `poly`

+ [Termite](https://github.com/thestinger/termite/): `termite`

+ [i3](https://github.com/i3/i3): `wm`

## Usage
`raven help` for a list of available commands:
```Commands:
help : show this screen
load [theme] : load a complete theme
new [theme] : create a new theme
delete [theme] : delete a theme
refresh : load last loaded theme
edit [theme] : initialize editing [theme]
add [option] [file] : add option to current theme
rm [option] : remove option from current theme
menu : show theme menu
```
## Configuration
A main config file is placed in ~/.config/raven/config, which has two options:
```
window_manager: |[window manager in use. Currently only i3 is supported.]|
monitor : |number of monitors available, to tell how many polybars to dupe.|
```

To configure a theme, start off by creating it with `raven new [theme]`. You'll automatically start editing that theme. Run `raven add [option] [file]` to add a specific option. Available options are:

+ poly(Polybar)
+ termite(Termite config)
+ wm(window manager config)
+ wall(Wallpaper)
+ xres(Xresources)
+ xres\_m(Xresources, to be merged)

If you place an i3 config named base\_i3 in ~/.config/raven, the contents of wm for a theme will be appended to it instead of being run on their own. This allows you to have a central config for keyboard shortcuts, and have cosmetics only be stored in the theme.
This will copy the indicated file to raven's registry, and run/reload/copy it when the edited theme is loaded or refreshed. Run `raven rm [option]` to remove an option from a theme.

### Polybar bar names
Up to two polybars will be started, depending on how many monitors you have configured. The bars should be named, in order: `main` and `other`  
