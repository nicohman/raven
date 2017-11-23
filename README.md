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
Currently WIP. There is no easy way to create new themes, but that shall quickly be remedied. Until then, `raven help` for a list of available commands:
```Commands:
help : show this screen
load [theme] : load a complete theme
new [theme] : create a new theme
delete [theme] : delete a theme
refresh : load last loaded theme
```
## Configuration
A Main config file is place in ~/.config/raven/config, which has two options:
```
window_manager: |[window manager in use. Currently only i3 is supported.]|
monitor : |number of monitors available, to tell how many polybars to dupe.|
```

To configure a theme, a specific format must be followed. This will be generated automatically later on, but for now run `raven new [theme]`. This will generate the basic framework in ~/.config/raven/themes. Within the theme file, place a | -deliminated list of 'options' you wish the theme to entail. Possible options right now are wm(for your configured wm), poly(polybar), xres(xresources), xres\_m(xresources to be merged, best for things like rofi configurations), wall(wallpaper), and termite(termite colorscheme/config). Within the same folder, put the config file to be used for each option, named that option. For instance, a wallpaper would be named `wall`, while a polybar config would be named `poly`.

