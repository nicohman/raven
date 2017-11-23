use std::fs;
use std::fs::DirEntry;
use std::fs::OpenOptions;
use std::io::Read;
//use std::os::unix::fs::OpenOptionsExt;
use std::env;
use std::io::Write;
use std::process::Command;
struct Theme {
    name: String,
    options: Vec<String>,
    wm: String,
}
impl Theme {
    fn load_wm(&self) {
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
        Command::new("i3-msg").arg("reload").spawn().expect(
            "Couldn't reload i3",
        );
    }
    fn load_termite(&self) {
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/termite",
            get_home() + "/.config/termite/config",
        ).expect("Couldn't overwrite termite config");
        Command::new("pkill")
            .arg("-SIGUSR1")
            .arg("termite")
            .spawn()
            .expect("Couldn't reload termite");
    }
    fn load_poly(&self, monitor: i32) {
        let order: Vec<&str> = vec!["main", "other"];
        for number in (0..monitor).rev() {
            println!("POLY");
            Command::new("polybar")
                .arg("-c")
                .arg(get_home() + "/.config/raven/themes/" + &self.name + "/poly")
                .arg(order[number as usize])
                .spawn()
                .expect("Failed to run polybar");
        }
    }
    fn load_wall(&self) {
        println!(
            "{}",
            get_home() + "/.config/raven/themes/" + &self.name + "/wall"
        );
        Command::new("feh")
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
        }
        let conf = get_config();
        let wm = String::from(conf.0.trim());
        let monitor = conf.1;
        let cmd = command.as_ref();
        if cmd == "load" || cmd == "refresh" {
            clear_prev();
        }
        match cmd {
            "load" => load_theme(&args[2], wm, monitor),
            "new" => new_theme(&args[2]),
            "help" => print_help(),
            "delete" => del_theme(&args[2]),
            "refresh" => refresh_theme(wm, monitor),
            _ => println!("Unknown command. raven help for commands."),
        }

    }
}
fn clear_prev(){
    Command::new("pkill").arg("polybar").spawn().unwrap();
}
fn del_theme(theme_name: &str) {
    fs::remove_dir_all(get_home() + "/.config/raven/themes/" + &theme_name)
        .expect("Couldn't delete theme");;
}
fn refresh_theme(wm: String, monitor: i32) {
    if fs::metadata(get_home() + "/.config/raven/last").is_err() {
        println!("No last theme saved. Cannot refresh.");
    } else {
        let mut contents = String::new();
        fs::File::open(get_home() + "/.config/raven/last")
            .expect("Couldn't open the last theme")
            .read_to_string(&mut contents)
            .expect("Couldn't read the last theme");
        load_theme(contents.trim(), wm, monitor);
    }
}
fn new_theme(theme_name: &str) {
    let res = fs::create_dir(get_home() + "/.config/raven/themes/" + &theme_name);
    if res.is_ok() {
        res.unwrap();
        println!(
            "{}",
            get_home() + "/.config/raven/themes/" + &theme_name + "/theme"
        );
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(
                get_home() + "/.config/raven/themes/" + &theme_name + "/theme",
            )
            .expect("can open");
        file.write_all((String::from("|")).as_bytes()).unwrap();
    } else {
        println!("Theme {} already exists", &theme_name);
    }
}
fn load_theme(theme_name: &str, wm: String, monitor: i32) {
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
            let new_theme = Theme {
                wm: String::from(wm.as_ref()),
                name: String::from(theme_name),
                options: options,
            };
            for option in &new_theme.options {
                println!("{}", &option);
                match option.as_ref() {
                    "poly" => new_theme.load_poly(monitor),
                    "wm" => new_theme.load_wm(),
                    "xres" => new_theme.load_xres(false),
                    "xres_m" => new_theme.load_xres(true),
                    "wall" => new_theme.load_wall(),
                    "termite" => new_theme.load_termite(),
                    _ => println!("Unknown option"),
                };
            }
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(get_home() + "/.config/raven/last")
                .expect("Couldn't open last theme file")
                .write_all(String::from(theme_name).as_bytes())
                .expect("Couldn't write to last theme file");
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
    file.write_all((String::from("window_manager: |i3|\n|monitor: |1|")).as_bytes())
        .unwrap();
    println!("Correctly initialized base config. Please run again to use raven.");
}
fn get_config() -> (String, i32) {
    let mut conf = String::new();
    fs::File::open(get_home() + "/.config/raven/config")
        .expect("Couldn't read config")
        .read_to_string(&mut conf)
        .unwrap();
    let conf_vec = conf.split('|').collect::<Vec<&str>>();
    (
        String::from(conf_vec[1].trim()),
        conf_vec[3].parse::<i32>().unwrap(),
    )
}
fn print_help() {
    println!("Commands:");
    println!("help : show this screen");
    println!("load [theme] : load a complete theme");
    println!("new [theme] : create a new theme");
    println!("delete [theme] : delete a theme");
    println!("refresh : load last loaded theme");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
