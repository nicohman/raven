use std::fs;
use std::fs::DirEntry;
use std::fs::OpenOptions;
use std::io::Read;
//use std::os::unix::fs::OpenOptionsExt;
use std::env;
use std::io::Write;
use std::collections::HashMap;
use std::process::Command;
struct Theme {
    name: String,
    options: Vec<String>,
    wm: String,
}
impl Theme {
    fn load_wm(&self) {
        let wm = &self.wm;
        match self.wm.as_ref() {
            "i3" => self.load_i3(),
            _ => println!("Unknown window manager"),
        }
    }
    fn load_i3(&self) {
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/wm",
            get_home() + "/.config/i3/config",
        ).expect("Couldn't overwrite i3 config");
        Command::new("sh")
            .arg("-c")
            .arg("i3-msg")
            .arg("reload")
            .spawn()
            .expect("Couldn't reload i3");
    }
    fn load_poly(&self) {
        let poly = Command::new("polybar")
            .arg("-c")
            .arg(get_home() + "/.config/raven/themes/" + &self.name + "/poly")
            .arg("main")
            .spawn()
            .expect("Failed to run polybar");
    }
    fn load_wall(&self) {
        println!("{}",get_home() + "/.config/raven/themes/" + &self.name + "/wall");
        let wall = Command::new("feh")
            .arg("--bg-scale")
            .arg(get_home() + "/.config/raven/themes/" + &self.name + "/wall")
            .spawn()
            .expect("Failed to change wallpaper");
    }
    fn load_xres(&self, merge: bool) {
        let mut xres = Command::new("xrdb");
        let mut name = String::from("xres");
        if merge {
            name.push_str("_m");
            xres.arg("-merge");
        }
        xres.arg(
            get_home() + "/.config/raven/themes/" + &self.name + "/" + &name,
        ).spawn()
            .expect("Could not run xrdb");
    }
}
fn main() {
    interpet_args();
}
fn interpet_args() {
    if fs::metadata(get_home() + "/.config/raven").is_err() ||
        fs::metadata(get_home() + "/.config/raven/config").is_err() ||
        fs::metadata(get_home() + "/.config/raven/themes").is_err()
    {
        init();
    } else {
        let args: Vec<String> = env::args().collect();
        let command: &str;
        if args.len() < 2 {
            command = "help";
        } else {
            command = &args[1];
            let wm = String::from(get_config().trim());
            match command.as_ref() {
                "load" => load_theme(&args[2], wm),
                "help" => print_help(),
                _ => println!("Unknown command. raven help for commands."),
            }
        }
    }
}

fn load_theme(theme_name: &str, wm: String) {
    if wm == String::from("i3") {
        println!("Using i3");
    }
    let entries = fs::read_dir(get_home() + "/.config/raven/themes/" + &theme_name)
        .expect("Can't read in theme directory");
    for entry in entries {
        let entry = proc_path(entry.unwrap());
        //println!("{}",entry);
        if String::from(entry).trim() == String::from("theme") {
            println!("Found theme {}", theme_name);
            let mut theme = String::new();
            fs::File::open(
                get_home() + "/.config/raven/themes/" + theme_name + "/theme",
            ).expect("Couldn't read theme")
                .read_to_string(&mut theme)
                .unwrap();
            let options = theme
                .split('|')
                .map(|x| String::from(String::from(x).trim()))
                .collect::<Vec<String>>();
            let mut new_theme = Theme {wm : String::from(wm.as_ref()),name : String::from(theme_name), options : options};
            for option in &new_theme.options {
                println!("{}", &option);
                match option.as_ref() {
                    "poly" => new_theme.load_poly(),
                    "wm" => new_theme.load_wm(),
                    "xres" => new_theme.load_xres(false),
                    "xres_m" => new_theme.load_xres(true),
                    "wall" => new_theme.load_wall(),
                    _ => println!("Unknown option"),
                };
            }
        }
    }
}
fn init() {
    fs::create_dir(get_home() + "/.config/raven").unwrap();
    fs::create_dir(get_home() + "/.config/raven/themes").unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/config")
        .unwrap();
    file.write_all((String::from("window_manager: |i3|\n")).as_bytes())
        .unwrap();
    println!("Correctly initialized base config. Please run again to use raven.");
}
fn get_config() -> (String) {
    let mut conf = String::new();
    fs::File::open(get_home() + "/.config/raven/config")
        .expect("Couldn't read config")
        .read_to_string(&mut conf)
        .unwrap();
    conf = String::from(conf.split('|').collect::<Vec<&str>>()[1]);
    conf
}
fn print_help() {
    println!("Commands:");
    println!("help : show this screen");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
