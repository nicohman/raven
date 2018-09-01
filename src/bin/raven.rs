use std::fs;
use std::fs::{DirEntry};
use std::env;
use std::process::Command;
use std::io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate multipart;
mod ravenserver;
mod ravenlib;
use ravenlib::rlib;
use ravenserver::ravens;
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate structopt;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "raven")]
enum Raven {
    #[structopt(name= "load", about = "Load a complete theme")]
    Load {
        theme:String
    },
    #[structopt(name = "new", about = "Create a new theme")]
    New {
        name:String
    },
    #[structopt(name = "modify", about = "Open the currently edited themes's option in $EDITOR")]    
    Modify {
        name:String
    },
    #[structopt(name = "delete", about = "Delete a theme")]        
    Delete {
        name:String
    },
    #[structopt(name = "info", about = "Print info about the theme being currently edited")]        
    Info {
    },
    #[structopt(name = "refresh", about = "Load last loaded theme")]        
    Refresh {
    },
    #[structopt(name = "install", about = "Install a theme from ThemeHub repo")]        
    Install {
        name:String
    },
    #[structopt(name = "add", about = "Add option to current theme")]        
    Add {
        option:String,
        name:String
    },
    #[structopt(name = "rm", about = "Remove an option from edited theme")]        
    Rm {
        name:String
    },
     #[structopt(name = "edit", about = "Edit theme")]        
    Edit {
        name:String
    },
    #[structopt(name = "menu", about = "Show theme menu")]        
    Menu {
    
    },

}
#[derive(StructOpt, Debug)]
enum Manage {
     #[structopt(name = "export", about = "Export a theme to a tarball")]        
    Export {
        name:String
    },
    #[structopt(name = "import", about = "Import a theme from a tarball")]        
    Import {
        name:String
    },
    #[structopt(name = "publish", about = "Publish an account online")]        
    Publish {
        name:String
    },
    #[structopt(name = "create", about = "Create an account")]        
    Create {
        name:String,
        pass1:String,
        pass2:String
    },
    #[structopt(name = "meta", about = "Edit an online theme's metadata")]        
    Meta {
        name:String,
        mtype:String,
        value:String
    },
    #[structopt(name = "delete_user", about = "Delete an online user's profile and owned themes")]            
    DUser {
        pass:String
    },
    #[structopt(name = "logout", about = "Log out of your user profile")]            
    Logout {
        
    },
    #[structopt(name = "unpublish", about = "Delete an online theme")]            
    Unpublish {
        name:String
    }

  
}
//Structure that holds theme data, to be stored in a theme folder.
fn main() {
    if rlib::check_init() {
        rlib::init();
    } else {
    interpet_args();
    }
}
fn interpet_args() {
    //Interpet arguments and check for a need to run init()
        let r = Raven::from_args();

        println!("{:?}",r);
        rlib::check_themes();
        //If a theme may be changing, kill the previous theme's processes. Currently only polybar
        //and lemonbar
        let conf = rlib::get_config();
        match r {
            Raven::Load{ theme } => {
                rlib::clear_prev();
                rlib::run_theme(rlib::load_theme(&theme).unwrap());
            },
            Raven::New{ name } => rlib::new_theme(&name),
            Raven::Modify{ name }  => modify_file(conf.editing,&name),
            Raven::Delete{ name }  => rlib::del_theme(&name),
            Raven::Edit{ name }  => rlib::edit(&name),
            // => manage_daemon(&args[2]),
            Raven::Info{ } => print_info(conf.editing),
          //  "manage" => process_manage_args(args.clone()),
            Raven::Refresh{ }  => {
                rlib::clear_prev();
                rlib::refresh_theme(conf.last);
            },
            Raven::Install{ name }  => ravens::download_theme(name),
            Raven::Add{ name, option }  => rlib::add_to_theme(&conf.editing, &option, &name),
            Raven::Rm{ name }  => rlib::rm_from_theme(&conf.editing, &name),
            Raven::Menu{ }  => show_menu(conf.menu_command),
            _ => println!("Unknown command. raven help for commands."),
        }

    }

fn process_manage_args(args: Vec<String>) {
    let cmd2 = (&args[2]).as_ref();
    if !check_args_cmd(args.len() - 3, cmd2) {
        println!("Not enough arguments for {}", &cmd2);
        ::std::process::exit(64);
    }
    match cmd2 {
        "export" => ravens::export(&args[3]),
        "import" => ravens::import(&args[3]),
        "publish" => ravens::upload_theme((&args[3]).to_string()),
        "create" => {
            ravens::create_user(
                (&args[3]).to_string(),
                (&args[4]).to_string(),
                (&args[5]).to_string(),
            )
        },
        "meta" => ravens::pub_metadata((&args[3]).to_string(),(&args[4]).to_string(),(&args[5]).to_string()),
        "unpublish" => ravens::unpublish_theme((&args[3]).to_string()),
        "login" => ravens::login_user((&args[3]).to_string(), (&args[4]).to_string()),
        "logout" => ravens::logout(),
        "delete_user" => ravens::delete_user((&args[3]).to_string()),
        _ => println!("Manage requires a subcommand. Run raven help for more info."),
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
}
fn check_args_cmd(num: usize, command: &str) -> bool {
    let need = match command {
        "load" => 1,
        "new" => 1,
        "rm" => 1,
        "modify" => 1,
        "edit" => 1,
        "add" => 2,
        "import" => 1,
        "export" => 1,
        "import" => 1,
        "meta" => 3,
        "create" => 3,
        "login" => 2,
        "delete_user" => 1,
        "unpublish" => 1,
        "publish" => 1,
        "install" => 1,
        "delete" => 1,
        _ => 0,
    };
    if num < need { false } else { true }
}
fn modify_file(editing: String, file: &str) {
    let editor = env::var_os("EDITOR").expect("Could not fetch $EDITOR from OS");
    let path = get_home() + "/.config/raven/themes/" + &editing + "/" + file;
    println!("Started {:?} at {}", editor, path);
    Command::new(editor).arg(path).spawn().expect(
        "Couldn't run $EDITOR",
    );
}
fn manage_daemon(command: &str) {
    let running = rlib::check_daemon();
    match command {
        "check" => {
            if running {
                println!("Cycle daemon running.");
            } else {
                println!("Cycle daemon not running.");
            }
        }
        "start" => {
            if !running {
                rlib::start_daemon();
            } else {
                println!("Cycle daemon already running.");
            }
        }
        "stop" => {
            if running {
                rlib::stop_daemon();
            } else {
                println!("Cycle daemon not running.");
            }
        }
        _ => {
            println!("Not a possible command.");
        }
    }
}
fn show_menu(menu_command: String) {
    let mut theme_list = String::new();
    let mut entries = fs::read_dir(get_home() + "/.config/raven/themes")
        .expect("Couldn't read themes")
        .collect::<Vec<io::Result<DirEntry>>>()
        .into_iter()
        .map(|x| proc_path(x.unwrap()))
        .collect::<Vec<String>>();
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
        rlib::clear_prev();
        let theme = rlib::load_theme(int_output.trim());
        if theme.is_err() {
            println!("Could not load in theme data. Does it exist?");
        } else {
            rlib::run_theme(theme.unwrap());
        }
    } else {
        println!("Theme not selected.");
    }

}

fn print_help() {
    println!("Commands:");
    println!("help : show this screen");
    println!("load [theme] : load a complete theme");
    println!("new [theme] : create a new theme");
    println!("delete [theme] : delete a theme");
    println!("refresh : load last loaded theme");
    println!("edit [theme] : initialize editing [theme]");
    println!("modify [option] : open the currently edited themes's [option] in $EDITOR");
    println!("add [option] [file] : add option to current theme");
    println!("rm [option] : remove option from current theme");
    println!("cycle {{check|start|stop}} : manage theme cycling daemon");
    println!("info : print info about the theme being currently edited");
    println!("menu : show theme menu");
    println!("install [name] : try to install a theme from the online repo");
    println!("manage [subcommand] : manage online theme publishing with subcommands");
    println!("      - import [archive] : import an exported theme");
    println!("      - export [theme] : export target theme to a tarball");
    println!("      - create [username] [password] [repeat password] : create a new user");
    println!("      - unpublish [name] : delete a published theme from repo");
    println!("      - login [username] [password] : login to a user profile");
    println!("      - publish [theme] : when logged in, publish a theme online");
    println!("      - logout : logout of a user profile");
    println!("      - meta [theme] [type] [value] : update the metadata of a published theme, either `screen`(a url to a screenshot) or `description`");
    println!("      - delete_user [password] : delete your user profile and any owned themes.");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
