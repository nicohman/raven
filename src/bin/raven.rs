use std::fs;
use std::fs::DirEntry;
use std::fs::OpenOptions;
use std::io::Read;
use std::env;
use std::io::Write;
use std::process::Command;
//Structure that holds all methods and data for individual themes.
struct Theme {
    name: String,
    options: Vec<String>,
    wm: String,
    monitor: i32,
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
        for number in (0..monitor) {
            let out = Command::new("sh").arg("-c").arg(String::from("polybar --config=")+&get_home() + "/.config/raven/themes/" + &self.name + "/poly "+order[number as usize]+" &")
                .spawn()
                .expect("Failed to run polybar");
            println!("{:?}",out);
        }
    }
    fn load_wall(&self) {
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
    //Interpet arguments and check for a need to run init()
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
        //If a theme may be changing, kill the previous theme's processes. Currently only polybar
        if cmd == "load" || cmd == "refresh" {
            clear_prev();
        }
        match cmd {
            "load" => run_theme(load_theme(&args[2], wm, monitor).unwrap()),
            "new" => new_theme(&args[2]),
            "help" => print_help(),
            "delete" => del_theme(&args[2]),
            "edit" => edit(&args[2]),
            "refresh" => refresh_theme(wm, monitor),
            "add" => add_to_theme(&get_editing(), &args[2], &args[3], wm, monitor),
            "rm" => rm_from_theme(&get_editing(), &args[2], wm, monitor),
            _ => println!("Unknown command. raven help for commands."),
        }

    }
}
fn edit(theme_name: &str) {
    //Add and rm commands will affect the theme you are currently editing
    if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name).is_ok() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/raven/editing")
            .expect("Can't open editing log")
            .write_all(theme_name.as_bytes())
            .unwrap();
        println!("You are now editing the theme {}", &theme_name);
    } else {
        println!("That theme does not exist");
    }
}
fn clear_prev() {
    Command::new("pkill").arg("polybar").spawn().unwrap();
}
fn del_theme(theme_name: &str) {
    fs::remove_dir_all(get_home() + "/.config/raven/themes/" + &theme_name)
        .expect("Couldn't delete theme");;
}
fn refresh_theme(wm: String, monitor: i32) {
    //Load last loaded theme
    if fs::metadata(get_home() + "/.config/raven/last").is_err() {
        println!("No last theme saved. Cannot refresh.");
    } else {
        let mut contents = String::new();
        fs::File::open(get_home() + "/.config/raven/last")
            .expect("Couldn't open the last theme")
            .read_to_string(&mut contents)
            .expect("Couldn't read the last theme");
        run_theme(load_theme(contents.trim(), wm, monitor).unwrap());
    }
}
fn new_theme(theme_name: &str) {
    //Create new theme directory and 'theme' file
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
        edit(&theme_name);
    } else {
        println!("Theme {} already exists", &theme_name);
    }
}
fn get_editing() -> String {
    let mut contents = String::new();
    fs::File::open(get_home() + "/.config/raven/editing")
        .expect("Couldn't open the currently being edited theme")
        .read_to_string(&mut contents)
        .expect("Couldn't read the currently being edited theme");
    contents
}
fn add_to_theme(theme_name: &str, option: &str, path: &str, wm: String, monitor: i32) {
    let mut cur_theme = load_theme(theme_name, wm, monitor).unwrap();
    let mut already_used = -1;
    cur_theme.options = cur_theme
        .options
        .iter()
        .map(|x| if x != option {
            x.to_owned()
        } else {
            already_used = 1;
            x.to_owned()
        })
        .collect::<Vec<String>>();
    if already_used == -1 {
        &cur_theme.options.push(String::from(option));
        let mut newop = cur_theme
            .options
            .iter()
            .filter(|x| x.len() > 0)
            .map(|x| String::from("|") + &x)
            .collect::<String>();
        newop.push('|');
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(
                get_home() + "/.config/raven/themes/" + &theme_name + "/theme",
            )
            .expect("can open");
        file.write_all(newop.as_bytes()).unwrap();
    }
    let mut totpath = env::current_dir().unwrap();
    totpath.push(path);
    fs::copy(
        totpath,
        get_home() + "/.config/raven/themes/" + &theme_name + "/" + &option,
    ).expect("Couldn't copy config in");
}
fn rm_from_theme(theme_name: &str, option: &str, wm: String, monitor: i32) {
    let mut cur_theme = load_theme(theme_name, wm, monitor).unwrap();
    let mut newop:String = cur_theme
        .options
        .iter()
        .filter(|x| x.len() > 0)
        .filter(|x| {
            let is = String::from(option).find(x.trim());
            is.is_none()
        }).map(|x| String::from("|")+&x).collect::<String>();
    newop.push('|');
    let theme_path = get_home() + "/.config/raven/themes/" + &theme_name + "/theme";
    fs::remove_file(&theme_path).expect("Couldn't reset");
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(
            theme_path
        )
        .expect("can open").write(newop.as_bytes()).expect("Couldn't write");
    fs::remove_file(
        get_home() + "/.config/raven/themes/" + &theme_name + "/" + &option,
    ).expect("Couldn't remove option");
}
fn run_theme(new_theme: Theme) {
    for option in &new_theme.options {
        println!("{} hi", &option);
        match option.to_lowercase().as_ref() {
            "poly" => new_theme.load_poly(new_theme.monitor),
            "wm" => new_theme.load_wm(),
            "xres" => new_theme.load_xres(false),
            "xres_m" => new_theme.load_xres(true),
            "wall" => new_theme.load_wall(),
            "termite" => new_theme.load_termite(),
            "|" => {}
            _ => println!("Unknown option"),
        };

    }
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/last")
        .expect("Couldn't open last theme file")
        .write_all(&new_theme.name.as_bytes())
        .expect("Couldn't write to last theme file");

}
fn load_theme(theme_name: &str, wm: String, monitor: i32) -> Result<Theme, &'static str> {
    //Load in data for and run loading methods for a specific theme
    if wm == String::from("i3") {
        println!("Using i3");
    }
    let mut new_theme: Theme = Theme {
        wm: String::from("i3"),
        monitor: 1,
        options: vec![String::from("no")],
        name: String::from("no"),
    };
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
                .map(|x| String::from(String::from(x).trim())).filter(|x| x.len() > 0)
                .collect::<Vec<String>>();
            println!("{}", options.len());
            new_theme = Theme {
                wm: String::from(wm.as_ref()),
                name: String::from(theme_name),
                options: options,
                monitor: monitor,
            };
        }
    }
    if new_theme.name != String::from("no") {
        Ok(new_theme)
    } else {
        Err("Can't find Theme data")
    }
}
fn init() {
    //Create base raven directories and config file(s)
    fs::create_dir(get_home() + "/.config/raven").unwrap();
    fs::create_dir(get_home() + "/.config/raven/themes").unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/config")
        .unwrap();
    file.write_all(
        (String::from("window_manager: |i3|\n|monitor: |1|")).as_bytes(),
    ).unwrap();
    println!("Correctly initialized base config. Please run again to use raven.");
}
fn get_config() -> (String, i32) {
    //Retrieve config settings from file
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
    println!("edit [theme] : initialize editing [theme]");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
