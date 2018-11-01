# Raven

A theme manager for linux, currently focusing on i3. Supports multiple different configuration files, and is fast and portable.

### Example

![A gif showing raven working](https://thumbs.gfycat.com/MenacingHandsomeCobra-size_restricted.gif)

## ThemeHub

Raven supports installing themes from and publishing themes to [ThemeHub](https://demenses.net), or your own instance of [ravenserver](https://github.com/nicohman/ravenserver). I encourage everyone to share their themes and rices there!

## Installation

All you technically require is [cargo](https://github.com/rust-lang/cargo) to be installed.
You can install from [crates.io](https://crates.io/crates/raventhemer) by running `cargo install raventhemer`, or by building manually:

Run:

`git clone https://github.com/nicohman/raven.git && cd raven`

`cargo install --path . --force`

The following packages are required for their relevant options:

+ [Polybar](https://github.com/jaagr/polybar): `poly`

+ [i3](https://github.com/i3/i3): `i3`

+ [Pywal](https://github.com/dylanaraps/pywal) : `pywal`

+ [Feh](https://github.com/derf/feh): `wall`

+ [Ncmpcpp](https://github.com/arybczak/ncmpcpp) : `ncmpcpp`

+ [Termite](https://github.com/thestinger/termite/): `termite`

+ [Bspwm](https://github.com/baskerville/bspwm) : `bspwm`

+ [Rofi](https://github.com/DaveDavenport/rofi) is used in the default config and is recommended. You can also add a custom rofi theme with the `rofi` option.

+ [Ranger](https://github.com/ranger/ranger) : `ranger`

+ [Lemonbar](https://github.com/LemonBoy/bar) : `lemonbar`

+ [Openbox](https://github.com/danakj/openbox) : `openbox`

+ [Dunst](https://github.com/dunst-project/dunst) : `dunst`

+ [Sublime Text 3](https://www.sublimetext.com/) : `sublt`

* New option suggestions are very welcome!

You can also download a prebuilt binary from [here](https://github.com/nicohman/raven/releases)

## Usage

`raven help` for a list of available commands:

```
raven
nicohman <nicohman@demenses.net>
A theme manager and switcher for desktop linux

USAGE:
    raven <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add        Add option to current theme
    cycle      Control cycle daemon
    delete     Delete a theme
    edit       Edit theme
    help       Prints this message or the help of the given subcommand(s)
    info       Print info about the theme being currently edited
    install    Install a theme from ThemeHub repo
    load       Load a complete theme
    manage     Manage online themes
    menu       Show theme menu
    modify     Open the currently edited themes's option in $EDITOR
    new        Create a new theme
    refresh    Load last loaded theme
    rm         Remove an option from edited theme
```

## Configuration

A main config file is placed in ~/.config/raven/config.json, which has the following options:

```
polybar: Array of strings, which decideds what bars are run with polybar. The size of the array should be the same as your monitor number
monitors : The number of monitors currently in use
menu_command: A command that, when raven menu is run, will be piped a list of theme names through STDIN and expects a theme name from STDOUT
last: The last theme raven loaded
editing: The theme you are currently editing
host: The URL of the ravenserver host to use. By default, [https://demenses.net](https://demenses.net)
```

To configure a theme, start off by creating it with `raven new [theme]`. You'll automatically start editing that theme. Run `raven add [option] [file]` to add a specific option. This will copy the indicated file to raven's registry, and run/reload/copy it when the edited theme is loaded or refreshed. Run `raven rm [option]` to remove an option from a theme. Available options are:

+ poly (Polybar)
+ i3 [base_i3] (i3 config)
+ xres (Xresources)
+ xres_m ( Xresources, to be merged)
+ pywal (an image file that will be used as the pywal base)
+ wall (Feh wallpaper)
+ ncmpcpp (ncmpcpp config file)
+ termite (Termite config)
+ script (An arbitrary executable file that will be run when this theme is loaded)
+ bspwm [base_bspwm] (bspwm config)
+ rofi (A rofi theme that will be copied to ~/.config/rofi/theme.rasi)
+ ranger (rc.conf)
+ lemonbar (A shell script that should spawn your lemonbar)
+ openbox [base_rc.xml] (rc.xml)
+ dunst [base_dunst] (dunstrc)
+ [sublt](#sublime-text-3) (sublt.json)

base_ files allow splitting the config from the cosmetics on the options with [base_]
For example if you place an i3 config named base\_i3 in ~/.config/raven, the contents of i3 for a theme will be appended to it instead of being run on their own. This allows you to have a central config for keyboard shortcuts, and have cosmetics only be stored in the theme.

The lemonbar option should be a shell script that runs lemonbar(s). They will automatically be killed just like polybars when the theme is changed or reloaded.

### Polybar bar names

As many polybars as you have monitors will be started. The names of the bars themselves should be configured in `config.json`. The default is ["main", "other"]. If you're sharing your themes with others, it is recommended that you leave the polybar monitor name blank, so that it automatically adapts to other monitor names.

### Cycle themes

With the cycle command you can control a daemon that will automatically cycle through all of your configured themes. You need to edit `~/.config/raven/time` and place the number of seconds there should be between each cycle into that file in order to use it.

### Sublime Text 3

To theme sublime text 3 you set the options inside a sublt.json.
An example sublt.json:

``` json
{
    "tmtheme": "false",
    "scs": "true",
    "sublimetheme": "DA.sublime-theme"
}
```

`tmtheme` (.tmTheme) and `scs` (.sublime-color-scheme) can be false, true or a path relative to the sublime text User folder (`~/.config/sublime-text-3/Packages/User`).
`sublimetheme` (.sublime-theme) on the other hand can only be false or a path.
`false` will disable the option.
`true` will copy the respective color scheme from the sublt folder into the sublime text User folder as raven.&lt;EXTENSION> and set it into `Preferences.sublime-config`.
`path` will set the path as the respective theme or colorscheme into `Preferences.sublime.config`.
