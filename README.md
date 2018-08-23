# Raven

A theme manager for linux, currently focusing on i3. Supports multiple different configuration files, and is fast and portable.

### Example

![A gif showing raven working](https://thumbs.gfycat.com/MenacingHandsomeCobra-size_restricted.gif)

## ThemeHub

Raven supports installing themes from and publishing themes to [ThemeHub](https://demenses.net). I encourage everyone to share their themes and rices there!

## Installation
All you technically require is [cargo](https://github.com/rust-lang/cargo) to be installed.
You can install from [crates.io](https://crates.io/crates/raventhemer) by running `cargo install raventhemer`, or by building manually:

Run:

`git clone https://github.com/nicohman/raven.git && cd raven`

`cargo build --release`

`sudo cp targets/release/raven /usr/bin/raven`

The following packages are required for their relevant options:

+ [Feh](https://github.com/derf/feh): `wall`

+ [Polybar](https://github.com/jaagr/polybar): `poly`

+ [Termite](https://github.com/thestinger/termite/): `termite`

+ [i3](https://github.com/i3/i3): `i3`

+ [Ranger](https://github.com/ranger/ranger) : `ranger`

+ [Lemonbar](https://github.com/ranger/ranger) : `lemonbar`

+ [Ncmpcpp](https://github.com/arybczak/ncmpcpp) : `ncmpcpp`

+ [Openbox](https://github.com/danakj/openbox) : `openbox`

+ [Pywal](https://github.com/dylanaraps/pywal) : `pywal`

+ [Bspwm](https://github.com/baskerville/bspwm) : `bspwm`

+ [Rofi](https://github.com/DaveDavenport/rofi) is used in the default config and is recommended. You can also add a custom rofi theme with the `rofi` option.

* New option suggestions are very welcome!

You can also download a prebuilt binary from [here](https://github.com/nicohman/raven/releases)

## Usage

`raven help` for a list of available commands:
```Commands:
help : show this screen
load [theme] : load a complete theme
new [theme] : create a new theme
delete [theme] : delete a theme
refresh : load last loaded theme
edit [theme] : initialize editing [theme]
modify [option] : open the currently edited themes's [option] in $EDITOR
add [option] [file] : add option to current theme
rm [option] : remove option from current theme
cycle {check|start|stop} : manage theme cycling daemon
info : print info about the theme being currently edited
menu : show theme menu
install [name] : try to install a theme from the online repo
manage [subcommand] : manage online theme publishing with subcommands
      - import [archive] : import an exported theme
      - export [theme] : export target theme to a tarball
      - create [username] [password] [repeat password] : create a new user
      - unpublish [name] : delete a published theme from repo
      - login [username] [password] : login to a user profile
      - publish [theme] : when logged in, publish a theme online
      - logout : logout of a user profile
      - meta [theme] [type] [value] : update the metadata of a published theme, either `screen`(a url to a screenshot) or `description`
      - delete_user [password] : delete your user profile and any owned themes.
```

## Configuration
A main config file is placed in ~/.config/raven/config.json, which has the following options:
```
polybar: Array of strings, which decideds what bars are run with polybar. The size of the array should be the same as your monitor number 
monitors : The number of monitors currently in use
menu_command: A command that, when raven menu is run, will be piped a list of theme names through STDIN and expects a theme name from STDOUT
last: The last theme raven loaded
editing: The theme you are currently editing
```

To configure a theme, start off by creating it with `raven new [theme]`. You'll automatically start editing that theme. Run `raven add [option] [file]` to add a specific option. This will copy the indicated file to raven's registry, and run/reload/copy it when the edited theme is loaded or refreshed. Run `raven rm [option]` to remove an option from a theme. Available options are:

+ poly(Polybar)
+ termite(Termite config)
+ i3(i3 config)
+ wall(Wallpaper)
+ xres(Xresources)
+ xres\_m(Xresources, to be merged)
+ ranger(rc.conf)
+ lemonbar(A shell script that should spawn your lemonbar)
+ ncmpcpp(ncmpcpp config file)
+ openbox(rc.xml)
+ script: An arbitrary executable file that will be run when this theme is loaded
+ pywal(an image file that will be used as the pywal base)
+ rofi(A rofi theme that will be copied to ~/.config/rofi/theme.rasi)

If you place an i3 config named base\_i3 in ~/.config/raven, the contents of i3 for a theme will be appended to it instead of being run on their own. This allows you to have a central config for keyboard shortcuts, and have cosmetics only be stored in the theme. This also applies for a file named base\_rc.xml, for openbox.

The lemonbar option should be a shell script that runs lemonbar(s). They will automatically be killed just like polybars when the theme is changed or reloaded.

### Polybar bar names
As many polybars as you have monitors will be started. The names of the bars themselves should be configured in `config.json`. The default is ["main", "other"]. If yu're sharing your themes with others, it is recommended that you leave the polybar monitor name blank, so that it automatically adapts to other monitor names.

### Cycle themes

With the cycle command you can control a daemon that will automatically cycle through all of your configured themes. You need to edit `~/.config/raven/time` and place the number of seconds there should be inbetween each cycle into that file in order to use it.
