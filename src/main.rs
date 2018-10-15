use std::fs;
use std::fs::DirEntry;
use std::env;
use std::process::Command;
use std::io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate multipart;
extern crate reqwest;
#[macro_use]
extern crate structopt;
use structopt::StructOpt;
pub mod ravenserver;
pub mod ravenlib;
pub mod args;
use args::*;
use ravenlib::*;
use ravenserver::*;
//Structure that holds theme data, to be stored in a theme folder.
fn main() {
    if check_init() {
        init();
    } else {
        interpet_args();
    }
}
fn interpet_args() {
    //Interpet arguments and check for a need to run init()
    let r = Raven::from_args();
    use Raven::*;
    use Manage::*;
    use Cycle::*;
    check_themes();
    //If a theme may be changing, kill the previous theme's processes. Currently only polybar
    //and lemonbar
    let conf = get_config();
    match r {
        Load { theme } => {
            clear_prev();
            run_theme(load_theme(&theme).unwrap());
        }
        New { name } => new_theme(&name),
        Modify { name, editor } => modify_file(conf.editing, &name, editor),
        Delete { name } => del_theme(&name),
        Edit { name } => edit(&name),
        ManageO { .. } => {
            match r {
                ManageO(Export { name }) => export(&name, check_tmp()),
                ManageO(Import { name }) => import(&name),
                ManageO(Publish { name }) => upload_theme(name),
                ManageO(Create { name, pass1, pass2 }) => create_user(name, pass1, pass2),
                ManageO(Unpublish { name }) => unpublish_theme(name),
                ManageO(Login { name, pass }) => login_user(name, pass),
                ManageO(Logout {}) => logout(),
                ManageO(DUser { pass }) => delete_user(pass),
                _ => println!("Well, this shouldn't be happening"),
            }
        }
        CycleD { .. } => {
            let running = check_daemon();
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
                        start_daemon();
                    } else {
                        println!("Cycle daemon already running.");
                    }
                }
                CycleD(Stop {}) => {
                    if running {
                        stop_daemon();
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
            clear_prev();
            refresh_theme(conf.last);
        }
        Install { name, force } => download_theme(name, force),
        Add { name, option } => add_to_theme(&conf.editing, &option, &name),
        Rm { name } => rm_from_theme(&conf.editing, &name),
        Menu {} => show_menu(conf.menu_command),
    }

}

fn print_info(editing: String) {
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
    let themes = get_themes(); 
    for t in themes {
        println!("{}", t);
    }
}
fn modify_file(editing: String, file: &str, editor: Option<String>) {
    //Pulls $EDITOR from environment variables
    if editor.is_none() {
        let editor = env::var_os("EDITOR");
        if editor.is_none() {
            println!("Could not fetch $EDITOR from your OS.");
            std::process::exit(64);
        }
    }
    let editor = editor.unwrap();
    let path = get_home() + "/.config/raven/themes/" + &editing + "/" + file;
    println!("Started {:?} at {}", editor, path);
    Command::new(editor).arg(path).spawn().expect(
        "Couldn't run $EDITOR",
    );
}
fn show_menu(menu_command: String) {
    let mut theme_list = String::new();
    let mut entries = get_themes();
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
        .arg(
            String::from("echo '") + &theme_list + "' | " + &menu_command,
        )
        .output()
        .expect("Failed to run menu.");
    let int_output = String::from_utf8_lossy(&output.stdout);
    if int_output.len() > 0 {
        clear_prev();
        let theme = load_theme(int_output.trim());
        if theme.is_err() {
            println!("Could not load in theme data. Does it exist?");
        } else {
            run_theme(theme.unwrap());
        }
    } else {
        println!("Theme not selected.");
    }

}

fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
