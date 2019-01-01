use std::{env, fs, fs::DirEntry, io, process::Command};
extern crate dirs;
extern crate ravenlib;
#[macro_use]
extern crate structopt;
use structopt::StructOpt;
#[macro_use]
extern crate human_panic;
pub mod args;
use args::*;
use dirs::home_dir;
use ravenlib::{config::*, daemon::*, ravenserver::*, themes::*};
fn main() {
    #[cfg(not(debug_assertions))]
    setup_panic!();
    if check_init() {
        init().unwrap();
    }
    interpet_args();
}
fn interpet_args() {
    //Interpet arguments and check for a need to run init()
    let r = Raven::from_args();
    use Cycle::*;
    use Manage::*;
    use Raven::*;
    check_themes().unwrap();
    //If a theme may be changing, kill the previous theme's processes. Currently only polybar
    //and lemonbar
    let conf = get_config().unwrap();
    match r {
        Load { theme } => {
            run_theme(&load_theme(theme).unwrap()).unwrap();
        }
        New { name } => new_theme(name).unwrap(),
        Modify { name, editor } => modify_file(conf.editing, name, editor),
        Delete { name } => del_theme(name).unwrap(),
        Edit { name } => {
            edit(name).unwrap();
        },
        Key {key, value} => key_value(key, value, conf.editing).unwrap(),
        ManageO { .. } => {
            match r {
                ManageO(Export { name }) => {
                    export(name, check_tmp()).unwrap();
                }
                ManageO(Import { name }) => import(name).unwrap(),
                ManageO(Publish { name }) => {
                    upload_theme(name).unwrap();
                },
                ManageO(Create { name, pass1, pass2 }) => {
                    create_user(name, pass1, pass2).unwrap();
                },
                ManageO(Unpublish { name }) => unpublish_theme(name).unwrap(),
                ManageO(Login { name, pass }) => login_user(name, pass).unwrap(),
                ManageO(Logout {}) => logout().unwrap(),
                ManageO(DUser { pass }) => delete_user(pass).unwrap(),
                _ => println!("Well, this shouldn't be happening"),
            };
        }
        CycleD { .. } => {
            let running = check_daemon().unwrap();
            match r {
                CycleD(Check {}) => {
                    if running {
                        println!("Cycle daemon running.");
                    } else {
                        println!("Cycle daemon not running.");
                    }
                }
                CycleD(Start {}) => {
                    if !running {
                        start_daemon().unwrap();
                    } else {
                        println!("Cycle daemon already running.");
                    }
                }
                CycleD(Stop {}) => {
                    if running {
                        stop_daemon().unwrap();
                    } else {
                        println!("Cycle daemon not running.");
                    }
                }
                _ => {
                    println!("Not a possible command.");
                }
            }
        }
        Info {} => print_info(conf.editing),
        Refresh {} => {
            refresh_theme(conf.last).unwrap();
        }
        Install { name, force } => {
            download_theme(name, force).unwrap();
        },
        Add { name, option } => add_to_theme(conf.editing, option, name).unwrap(),
        Rm { name } => rm_from_theme(conf.editing, name).unwrap(),
        Menu {} => show_menu(conf.menu_command),
    };
}

fn print_info<N>(editing: N)
where
    N: Into<String>,
{
    let editing = editing.into();
    let options = fs::read_dir(get_home() + "/.config/raven/themes/" + &editing)
        .expect("Couldn't read themes")
        .collect::<Vec<io::Result<DirEntry>>>()
        .into_iter()
        .map(|x| proc_path(x.unwrap()))
        .filter(|x| x != "theme.json")
        .collect::<Vec<String>>();
    println!("Current configured options for {}", editing);
    for option in options {
        println!("{}", option);
    }
    println!("All themes: ");
    let themes = get_themes().unwrap();
    for t in themes {
        println!("{}", t);
    }
}
fn modify_file<N>(editing: N, file: N, editor: Option<N>)
where
    N: Into<String>,
{
    //Pulls $EDITOR from environment variables
    if editor.is_none() {
        let editor = env::var_os("EDITOR");
        if editor.is_none() {
            println!("Could not fetch $EDITOR from your OS.");
            std::process::exit(64);
        }
    }
    let editor = editor.unwrap().into();
    let path = get_home() + "/.config/raven/themes/" + &editing.into() + "/" + &file.into();
    println!("Started {:?} at {}", editor, path);
    Command::new(editor)
        .arg(path)
        .spawn()
        .expect("Couldn't run $EDITOR");
}
fn show_menu<N>(menu_command: N)
where
    N: Into<String>,
{
    let mut theme_list = String::new();
    let mut entries = get_themes().unwrap();
    entries.sort_by(|a, b| a.cmp(&b));
    let mut i = 0;
    for entry in entries {
        if entry.chars().count() > 0 {
            if i > 0 {
                theme_list.push_str("\n");
            }
            theme_list.push_str(&entry);
            i += 1;
        }
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(String::from("echo '") + &theme_list + "' | " + &menu_command.into())
        .output()
        .expect("Failed to run menu.");
    let int_output = String::from_utf8_lossy(&output.stdout);
    if int_output.len() > 0 {
        let theme = load_theme(int_output.trim());
        if theme.is_err() {
            println!("Could not load in theme data. Does it exist?");
        } else {
            run_theme(&theme.unwrap()).unwrap();
        }
    } else {
        println!("Theme not selected.");
    }
}

fn get_home() -> String {
    return String::from(home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
