use std::fs;
use std::fs::{File, OpenOptions, DirEntry};
use std::io::Read;
use std::env;
use std::io::Write;
use std::process::Command;
use std::io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate multipart;
mod ravenserver;
use ravenserver::ravens;
extern crate hyper;
extern crate reqwest;
//Structure that holds theme data, to be stored in a theme folder.
#[derive(Serialize, Deserialize, Debug)]
struct ThemeStore {
    name: String,
    options: Vec<String>,
    enabled: Vec<String>,
}
//Structure that holds all methods and data for individual themes.
struct Theme {
    name: String,
    options: Vec<String>,
    monitor: i32,
    enabled: Vec<String>,
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
                "pywal" => self.load_pywal(),
                "wall" => self.load_wall(),
                "ncmpcpp" => self.load_ncm(),
                "termite" => self.load_termite(),
                "script" => self.load_script(),
                "bspwm" => self.load_bspwm(),
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
    fn load_pywal(&self) {
        let arg = get_home() + "/.config/raven/themes/" + &self.name + "/pywal";
        Command::new("wal")
            .arg("-n")
            .arg("-i")
            .arg(arg)
            .output()
            .expect("Couldn't run pywal");
    }
    fn load_script(&self) {
        Command::new(
            get_home() + "/.config/raven/themes/" + &self.name + "/script",
        ).output()
            .expect("Couldn't run custom script");
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
            .read_to_string(&mut rest)
            .unwrap();
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
            .output()
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
    fn load_bspwm(&self) {
        let mut config = String::new();
        if fs::metadata(get_home() + "/.config/raven/base_bspwm").is_ok() {
            fs::File::open(get_home() + "/.config/raven/base_bspwm")
                .unwrap()
                .read_to_string(&mut config)
                .unwrap();
        }
        let mut app = String::new();
        fs::File::open(
            get_home() + "/.config/raven/themes/" + &self.name + "/bspwm",
        ).unwrap()
            .read_to_string(&mut app)
            .unwrap();

        config.push_str(&app);
        fs::remove_file(get_home() + "/.config/bspwm/bspwmrc").unwrap();
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/bspwm/bspwmrc")
            .expect("Couldn't open bspwmrc file")
            .write_all(config.as_bytes())
            .unwrap();
        Command::new("bspc").arg("reload").output().expect(
            "Couldn't reload bspwm",
        );

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
        Command::new("i3-msg").arg("reload").output().expect(
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
            .output()
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
            .output()
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
        ).output()
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
        check_themes();
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
        //and lemonbar
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
            "manage" => process_manage_args(args.clone()),
            "refresh" => refresh_theme(conf.last),
            "install" => ravens::download_theme((&args[2]).to_string()),
            "add" => add_to_theme(&conf.editing, &args[2], &args[3]),
            "rm" => rm_from_theme(&conf.editing, &args[2]),
            "menu" => show_menu(conf.menu_command),
            _ => println!("Unknown command. raven help for commands."),
        }

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
        }
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
fn edit(theme_name: &str) {
    //Add and rm commands will affect the theme you are currently editing
    if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name).is_ok() {
        let mut conf = get_config();
        conf.editing = theme_name.to_string();
        up_config(conf);
        println!("You are now editing the theme {}", &theme_name);
    } else {
        println!("That theme does not exist");
    }
}
fn clear_prev() {
    Command::new("pkill").arg("polybar").output().unwrap();
    Command::new("pkill").arg("lemonbar").output().unwrap();
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
    let cur_theme = load_theme(theme_name).unwrap();
    let mut new_themes = ThemeStore {
        name: theme_name.to_string(),
        options: cur_theme.options,
        enabled: cur_theme.enabled,
    };
    let mut already_used = false;
    for opt in &new_themes.options {
        if opt == option {
            already_used = true;
        }
    }
    if !already_used {
        new_themes.options.push(String::from(option));
        up_theme(new_themes);
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
    let mut new_themes = ThemeStore {
        name: theme_name.to_string(),
        options: cur_theme.options,
        enabled: cur_theme.enabled,
    };
    let mut found = false;
    let mut i = 0;
    while i < new_themes.options.len() {
        if &new_themes.options[i] == option {
            println!("Found option {}", option);
            found = true;
            new_themes.options.remove(i);
        }
        i += 1;
    }
    if found {
        up_theme(new_themes);
    } else {
        println!("Couldn't find option {}", option);
    }
}
fn run_theme(new_theme: Theme) {
    //Run/refresh a loaded Theme
    new_theme.load_all();
    let mut conf = get_config();
    conf.last = new_theme.name;
    up_config(conf);
}
fn up_config(conf: Config) {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/~config.json")
        .expect("Couldn't open last theme file")
        .write_all(serde_json::to_string(&conf).unwrap().as_bytes())
        .expect("Couldn't write to last theme file");
    fs::copy(
        get_home() + "/.config/raven/~config.json",
        get_home() + "/.config/raven/config.json",
    ).unwrap();
    fs::remove_file(get_home() + "/.config/raven/~config.json").unwrap();
}
fn up_theme(theme: ThemeStore) {
    let wthemepath = get_home() + "/.config/raven/themes/" + &theme.name + "/~theme.json";
    let themepath = get_home() + "/.config/raven/themes/" + &theme.name + "/theme.json";
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(&wthemepath)
        .expect("Couldn't open theme file")
        .write_all(serde_json::to_string(&theme).unwrap().as_bytes())
        .expect("Couldn't write to theme file");
    fs::copy(&wthemepath, &themepath).unwrap();
    fs::remove_file(&wthemepath).unwrap();
}

fn convert_theme(theme_name: &str) {
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
        .filter(|x| x != "|")
        .collect::<Vec<String>>();
    let themes = ThemeStore {
        name: theme_name.to_string(),
        enabled: Vec::new(),
        options: options,
    };
    fs::remove_file(
        get_home() + "/.config/raven/themes/" + theme_name + "/theme",
    ).unwrap();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(
            get_home() + "/.config/raven/themes/" + theme_name + "/theme.json",
        )
        .expect("Can't open theme.json")
        .write_all(serde_json::to_string(&themes).unwrap().as_bytes())
        .unwrap();
}
fn check_themes() {
    let entries = fs::read_dir(get_home() + "/.config/raven/themes").unwrap();
    for entry in entries {
        let entry = proc_path(entry.unwrap());
        if fs::metadata(get_home() + "/.config/raven/themes/" + &entry + "/theme").is_ok() {
            convert_theme(&entry);
        }
    }
}
fn load_theme(theme_name: &str) -> Result<Theme, &'static str> {
    //Load in data for and run loading methods for a specific theme
    let conf = get_config();
    let ent_res = fs::read_dir(get_home() + "/.config/raven/themes/" + &theme_name);
    if ent_res.is_ok() {
        println!("Found theme {}", theme_name);
        if fs::metadata(
            get_home() + "/.config/raven/themes/" + &theme_name + "/theme.json",
        ).is_ok()
        {
            let mut theme = String::new();
            fs::File::open(
                get_home() + "/.config/raven/themes/" + theme_name + "/theme.json",
            ).expect("Couldn't read theme")
                .read_to_string(&mut theme)
                .unwrap();
            let theme_info: ThemeStore = serde_json::from_str(&theme).unwrap();
            let opts: Vec<String> = theme_info.options;
            let new_theme = Theme {
                name: String::from(theme_name),
                options: opts,
                monitor: conf.monitors,
                enabled: theme_info.enabled,
                order: conf.polybar.clone(),
            };
            Ok(new_theme)
        } else {

            Err("Can't find Theme data")
        }
    } else {
        println!("Theme does not exist.");
        ::std::process::exit(64);
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
    println!("install [name] : try to install a theme from the online repo");
    println!("manage [subcommand] : manage online theme publishing with subcommands");
    println!("      - import [archive] : import an exported theme");
    println!("      - export [theme] : export target theme to a tarball");
    println!("      - create [username] [password] [repeat password] : create a new user");
    println!("      - unpublish [name] : delete a published theme from repo");
    println!("      - login [username] [password] : login to a user profile");
    println!("      - publish [theme] : when logged in, publish a theme online");
    println!("      - logout : logout of a user profile");
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
