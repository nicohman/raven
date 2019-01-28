use std::{env, fs, fs::DirEntry, io, process::Command};
extern crate dirs;
extern crate ravenlib;
#[macro_use]
extern crate log;
extern crate clap_verbosity_flag;
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
        Load { theme, verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            run_theme(&load_theme(theme).unwrap()).unwrap();
        }
        New { name, verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            new_theme(name).unwrap()
        }
        Modify {
            name,
            editor,
            verbose,
        } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            modify_file(conf.editing, name, editor)
        }
        Delete { name, verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            del_theme(name).unwrap()
        }
        Edit { name, verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            edit(name).unwrap();
        }
        Key {
            key,
            value,
            verbose,
        } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            key_value(key, value, conf.editing).unwrap()
        }
        ManageO { .. } => {
            match r {
                ManageO(Export { name, verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    export(name, check_tmp()).unwrap();
                }
                ManageO(Import { name, verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    import(name).unwrap()
                }
                ManageO(Publish { name, verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    upload_theme(name).unwrap();
                }
                ManageO(Create {
                    name,
                    pass1,
                    pass2,
                    verbose,
                }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    create_user(name, pass1, pass2).unwrap();
                }
                ManageO(Unpublish { name, verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    unpublish_theme(name).unwrap()
                }
                ManageO(Login {
                    name,
                    pass,
                    verbose,
                }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    login_user(name, pass).unwrap()
                }
                ManageO(Logout { verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    logout().unwrap()
                }
                ManageO(DUser { pass, verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    delete_user(pass).unwrap()
                }
                _ => println!("Well, this shouldn't be happening"),
            };
        }
        CycleD { .. } => {
            let running = check_daemon().unwrap();
            match r {
                CycleD(Check { verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    if running {
                        println!("Cycle daemon running.");
                    } else {
                        println!("Cycle daemon not running.");
                    }
                }
                CycleD(Start { verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
                    if !running {
                        start_daemon().unwrap();
                    } else {
                        println!("Cycle daemon already running.");
                    }
                }
                CycleD(Stop { verbose }) => {
                    verbose
                        .setup_env_logger("raven")
                        .expect("Couldn't set up logger");
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
        Info { verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            print_info(conf.editing)
        }
        Refresh { verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            refresh_theme(conf.last).unwrap();
        }
        Install {
            name,
            force,
            verbose,
        } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            download_theme(name, force).unwrap();
        }
        Add {
            name,
            option,
            verbose,
        } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            add_to_theme(conf.editing, option, name).unwrap()
        }
        Rm { name, verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            rm_from_theme(conf.editing, name).unwrap()
        }
        Menu { verbose } => {
            verbose
                .setup_env_logger("raven")
                .expect("Couldn't set up logger");
            show_menu(conf.menu_command)
        }
    };
}

fn print_info<N>(editing: N)
where
    N: Into<String>,
{
    let editing = editing.into();
    info!("Reading in themes");
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
            error!("Could not fetch $EDITOR from your OS.");
            std::process::exit(64);
        }
    }
    let editor = editor.unwrap().into();
    let path = get_home() + "/.config/raven/themes/" + &editing.into() + "/" + &file.into();
    info!("Started editor {:?} at {}", editor, path);
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
    info!("Starting menu command");
    let output = Command::new("sh")
        .arg("-c")
        .arg(String::from("echo '") + &theme_list + "' | " + &menu_command.into())
        .output()
        .expect("Failed to run menu.");
    info!("Menu command stopped. Parsing into theme name.");
    let int_output = String::from_utf8_lossy(&output.stdout);
    if int_output.len() > 0 {
        info!("loading selected theme");
        let theme = load_theme(int_output.trim());
        if theme.is_err() {
            error!("Could not load in theme data. Does it exist?");
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
