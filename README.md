# Raven [![](https://img.shields.io/crates/v/raventhemer.svg?style=flat-square)](https://crates.io/crates/raventhemer) [![builds.sr.ht status](https://builds.sr.ht/~nicohman/raven.svg)](https://builds.sr.ht/~nicohman/raven?)

A theme manager for linux, currently focusing on i3. Supports multiple different configuration files, and is fast and portable. Upstream now hosted at [sr.ht](https://git.sr.ht/~nicohman/raven). You can find [ravenlib](https://git.sr.ht/~nicohman/ravenlib) there as well, if you're looking to add to or build off of the core features.

### Example

![A gif showing raven working](https://thumbs.gfycat.com/MenacingHandsomeCobra-size_restricted.gif)

## ThemeHub

Raven supports installing themes from and publishing themes to [ThemeHub](https://demenses.net), or your own instance of [ravenserver](https://git.sr.ht/~nicohman/ravenserver). I encourage everyone to share their themes and rices there!

## Wiki

There's a wiki [here](https://man.sr.ht/~nicohman/raven), which provides more in-depth information on raven and the surrounding projects.

## Getting Started

If you just want to get going, you can install raven from [crates.io](https://crates.io/crates/raventhemer) with

`cargo install raventhemer`

If you don't want to install cargo, you can download a binary built from the latest git commit [here](https://demenses,net/downloads).

## Installation

All you technically require is [cargo](https://github.com/rust-lang/cargo) to be installed.
You can install from [crates.io](https://crates.io/crates/raventhemer) by running `cargo install raventhemer`, or by building manually:

Run:

`git clone https://git.sr.ht/~nicohman/raven && cd raven`

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

+ [Sublime Text 3](https://www.sublimetext.com/) : `st_tmtheme`, `st_scs` and `st_subltheme`

+ [VSCode](https://github.com/Microsoft/vscode) : `vscode`

* New option suggestions are very welcome!

You can also download a(possibly outdated prebuilt binary from [here](https://github.com/nicohman/raven/releases), or a binary built from the latest git commit at [my website](https://demenses.net/downloads).

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

The section on configuring raven has been moved to the [wiki](https://man.sr.ht/~nicohman/raven). Go check it to learn how to configure raven!
