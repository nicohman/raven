use std::fs;
use std::fs::DirEntry;
use std::fs::OpenOptions;
use std::io::Read;
use std::env;
use std::io::Write;
use std::process::Command;
use std::io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//Structure that holds all methods and data for individual themes.
struct Theme {
    name: String,
    options: Vec<String>,
    monitor: i32,
    order: Vec<String>,
}
//Config structure for holding all main config options
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    monitors: i32,
    polybar: Vec<String>,
    menu_command: String,
    last: String,
    editing: String,
}
impl Config {
    fn default() -> Config {
        Config {
            monitors: 1,
            polybar: vec!["main".to_string(), "other".to_string()],
            menu_command: "rofi -theme sidebar -mesg 'raven:' -p '> ' -dmenu".to_string(),
            last: "".to_string(),
            editing: "".to_string(),
        }
    }
}
impl Theme {
    fn load_all(&self) {
        for option in &self.options {
            match option.to_lowercase().as_ref() {
                "poly" => self.load_poly(self.monitor),
                "wm" => self.load_i3(true),
                "i3" => self.load_i3(false),
                "xres" => self.load_xres(false),
                "xres_m" => self.load_xres(true),
                "wall" => self.load_wall(),
                "ncmpcpp" => self.load_ncm(),
                "termite" => self.load_termite(),
                "ranger" => self.load_ranger(),
                "lemonbar" => self.load_lemon(),
                "openbox" => self.load_openbox(),
                "|" => {}
                _ => println!("Unknown option"),
            };
            if !option.contains("|") {
                println!("Loaded option {}", option);
            }

        }
        println!("Loaded all options for theme {}", self.name);

    }
    fn load_openbox(&self) {
        let mut base = String::new();
        if fs::metadata(get_home() + "/.config/raven/base_rc.xml").is_ok() {
            fs::File::open(get_home() + "/.config/raven/base_rc.xml")
                .unwrap()
                .read_to_string(&mut base)
                .unwrap();

        }
        let mut rest = String::new();
        fs::File::open(
            get_home() + "/.config/raven/themes/" + &self.name + "/openbox",
        ).unwrap()
            .read_to_string(&mut rest).unwrap();
        base.push_str(&rest);
        fs::remove_file(get_home() + "/.config/openbox/rc.xml").unwrap();
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/openbox/rc.xml")
            .expect("Couldn't open rc.xml")
            .write_all(base.as_bytes())
            .unwrap();
        Command::new("openbox")
            .arg("--reconfigure")
            .spawn()
            .expect("Couldn't reload openbox");
    }
    fn load_ranger(&self) {
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/ranger",
            get_home() + "/.config/ranger/rc.conf",
        ).expect("Couldn't overwrite ranger config");
    }
    fn load_ncm(&self) {
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/ncmpcpp",
            get_home() + "/.ncmpcpp/config",
        ).expect("Couldn't overwrite ncmpcpp config");

    }
    fn load_i3(&self, isw: bool) {
        let mut config = String::new();
        if fs::metadata(get_home() + "/.config/raven/base_i3").is_ok() {
            fs::File::open(get_home() + "/.config/raven/base_i3")
                .unwrap()
                .read_to_string(&mut config)
                .unwrap();
        }
        let mut app = String::new();
        if isw {
            fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/wm")
                .unwrap()
                .read_to_string(&mut app)
                .unwrap();
        } else {
            fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/i3")
                .unwrap()
                .read_to_string(&mut app)
                .unwrap();
        }
        config.push_str(&app);
        fs::remove_file(get_home() + "/.config/i3/config").unwrap();
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/i3/config")
            .expect("Couldn't open i3 file")
            .write_all(config.as_bytes())
            .unwrap();
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
        for number in 0..monitor {
            Command::new("sh")
                .arg("-c")
                .arg(
                    String::from("polybar --config=") + &get_home() +
                        "/.config/raven/themes/" + &self.name + "/poly " +
                        &self.order[number as usize] + " > /dev/null 2> /dev/null",
                )
                .spawn()
                .expect("Failed to run polybar");
        }
    }
    fn load_lemon(&self) {
        Command::new("sh")
            .arg(
                get_home() + "/.config/raven/themes/" + &self.name + "/lemonbar",
            )
            .spawn()
            .expect("Failed to run lemonbar script");
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
        fs::metadata(get_home() + "/.config/raven/config.json").is_err() ||
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
        let cmd = command.as_ref();
        if args.len() > 1 {
            if !check_args_cmd(args.len() - 2, cmd) {
                println!("Not enough arguments for {}", &cmd);
                ::std::process::exit(64);
            }
        }

        //If a theme may be changing, kill the previous theme's processes. Currently only polybar
        if cmd == "load" || cmd == "refresh" {
            clear_prev();
        }
        match cmd {
            "load" => run_theme(load_theme(&args[2]).unwrap()),
            "new" => new_theme(&args[2]),
            "help" => print_help(),
            "modify" => modify_file(conf.editing, &args[2]),
            "delete" => del_theme(&args[2]),
            "edit" => edit(&args[2]),
            "cycle" => manage_daemon(&args[2]),
            "info" => print_info(conf.editing),
            "refresh" => refresh_theme(conf.last),
            "add" => add_to_theme(&conf.editing, &args[2], &args[3]),
            "rm" => rm_from_theme(&conf.editing, &args[2]),
            "menu" => show_menu(conf.menu_command),
            _ => println!("Unknown command. raven help for commands."),
        }

    }
}
fn print_info(editing: String) {
    let options = fs::read_dir(get_home() + "/.config/raven/themes/" + &editing)
        .expect("Couldn't read themes")
        .collect::<Vec<io::Result<DirEntry>>>()
        .into_iter()
        .map(|x| proc_path(x.unwrap()))
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
        "delete" => 1,
        _ => 0,
    };
    if num < need { false } else { true }
}
fn modify_file(editing: String, file: &str) {
    let editor = env::var_os("EDITOR").expect("Could not fetch $EDITOR from OS");
    let path = get_home() + "/.config/raven/themes/" + &editing + "/" + file;
    println!("{}", path);
    Command::new(editor).arg(path).spawn().expect(
        "Couldn't run $EDITOR",
    );
}
fn start_daemon() {
    Command::new("sh").arg("-c").arg("ravend").spawn().expect(
        "Couldn't start daemon.",
    );
    println!("Started cycle daemon.");

}
fn stop_daemon() {
    Command::new("pkill")
        .arg("-SIGKILL")
        .arg("ravend")
        .output()
        .expect("Couldn't stop daemon.");
    println!("Stopped cycle daemon.");
}
fn check_daemon() -> bool {
    let out = Command::new("ps").arg("aux").output().expect(
        "Couldn't find daemon",
    );
    let form_out = String::from_utf8_lossy(&out.stdout);
    let line_num = form_out.lines().filter(|x| x.contains("ravend")).count();
    if line_num > 0 { true } else { false }
}
fn manage_daemon(command: &str) {
    let running = check_daemon();
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
                start_daemon();
            } else {
                println!("Cycle daemon already running.");
            }
        }
        "stop" => {
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
fn show_menu(menu_command: String) {
    let mut theme_list = String::new();
    let mut entries = fs::read_dir(get_home() + "/.config/raven/themes")
        .expect("Couldn't read themes")
        .collect::<Vec<io::Result<DirEntry>>>()
        .into_iter()
        .map(|x| proc_path(x.unwrap()))
        .collect::<Vec<String>>();
    entries.sort_by(|a, b| a.cmp(&b));
    for entry in entries {
        theme_list.push_str(&entry);
        theme_list.push_str("\n");
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
fn edit(theme_name: &str) {
    //Add and rm commands will affect the theme you are currently editing
    if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name).is_ok() {
        let mut conf = get_config();
        conf.editing = theme_name.to_string();
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/raven/config.json")
            .expect("Can't open editing log")
            .write_all(serde_json::to_string(&conf).unwrap().as_bytes())
            .unwrap();
        println!("You are now editing the theme {}", &theme_name);
    } else {
        println!("That theme does not exist");
    }
}
fn clear_prev() {
    Command::new("pkill").arg("polybar").spawn().unwrap();
    Command::new("pkill").arg("lemonbar").spawn().unwrap();
}
fn del_theme(theme_name: &str) {
    fs::remove_dir_all(get_home() + "/.config/raven/themes/" + &theme_name)
        .expect("Couldn't delete theme");;
}
fn refresh_theme(last: String) {
    //Load last loaded theme
    if last.chars().count() > 0 {
        run_theme(load_theme(last.trim()).unwrap());
    } else {

        println!("No last theme saved. Cannot refresh.");
    }
}
fn new_theme(theme_name: &str) {
    //Create new theme directory and 'theme' file
    let res = fs::create_dir(get_home() + "/.config/raven/themes/" + &theme_name);
    if res.is_ok() {
        res.unwrap();
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
fn add_to_theme(theme_name: &str, option: &str, path: &str) {
    //Add an option to a theme
    let mut cur_theme = load_theme(theme_name).unwrap();
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
fn rm_from_theme(theme_name: &str, option: &str) {
    //Remove an option from a theme
    let cur_theme = load_theme(theme_name).unwrap();
    let mut newop: String = cur_theme
        .options
        .iter()
        .filter(|x| x.len() > 0)
        .filter(|x| {
            let is = String::from(option).find(x.trim());
            is.is_none()
        })
        .map(|x| String::from("|") + &x)
        .collect::<String>();
    newop.push('|');
    let theme_path = get_home() + "/.config/raven/themes/" + &theme_name + "/theme";
    fs::remove_file(&theme_path).expect("Couldn't reset");
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(theme_path)
        .expect("can open")
        .write(newop.as_bytes())
        .expect("Couldn't write");
    fs::remove_file(
        get_home() + "/.config/raven/themes/" + &theme_name + "/" + &option,
    ).expect("Couldn't remove option");
}
fn run_theme(new_theme: Theme) {
    //Run/refresh a loaded Theme
    new_theme.load_all();
    let mut conf = get_config();
    conf.last = new_theme.name;
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/config.json")
        .expect("Couldn't open last theme file")
        .write_all(serde_json::to_string(&conf).unwrap().as_bytes())
        .expect("Couldn't write to last theme file");

}
fn load_theme(theme_name: &str) -> Result<Theme, &'static str> {
    //Load in data for and run loading methods for a specific theme
    let conf = get_config();
    let mut new_theme: Theme = Theme {
        monitor: 1,
        options: vec![String::from("no")],
        name: String::from("no"),
        order: conf.polybar.clone(),
    };
    let ent_res = fs::read_dir(get_home() + "/.config/raven/themes/" + &theme_name);
    if ent_res.is_ok() {
        let entries = ent_res.unwrap();
        for entry in entries {
            let entry = proc_path(entry.unwrap());
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
                    .filter(|x| x.len() > 0)
                    .collect::<Vec<String>>();
                new_theme = Theme {
                    name: String::from(theme_name),
                    options: options,
                    monitor: conf.monitors,
                    order: conf.polybar.clone(),
                };
            }
        }
    } else {
        println!("Theme does not exist.");
        ::std::process::exit(64);
    }
    if new_theme.name != String::from("no") {
        Ok(new_theme)
    } else {
        Err("Can't find Theme data")
    }
}
fn init() {
    //Create base raven directories and config file(s)
    if fs::metadata(get_home() + "/.config/raven/config").is_err() {
        fs::create_dir(get_home() + "/.config/raven").unwrap();
        fs::create_dir(get_home() + "/.config/raven/themes").unwrap();
    } else {
        println!(
            "The config file format has changed. Please check ~/.config/raven/config.json to reconfigure raven."
        );
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/config.json")
        .unwrap();
    let default = serde_json::to_string(&Config::default()).unwrap();
    file.write_all(default.as_bytes()).unwrap();
    println!("Correctly initialized base config. Please run again to use raven.");
}
fn get_config() -> Config {
    //Retrieve config settings from file
    let mut conf = String::new();
    fs::File::open(get_home() + "/.config/raven/config.json")
        .expect("Couldn't read config")
        .read_to_string(&mut conf)
        .unwrap();
    let config: Config = serde_json::from_str(&conf).expect("Couldn't read config file");
    config
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
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
