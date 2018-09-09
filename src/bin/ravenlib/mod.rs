
pub mod rlib {
    use std::fs;
    use std::fs::{OpenOptions, DirEntry};
    use std::os::unix::fs::OpenOptionsExt;
    use std::io::Read;
    use std::env;
    use std::io::Write;
    use serde_json;
    use std::process::Command;
    use std::io;
    //Returns home directory as string
    fn get_home() -> String {
        return String::from(env::home_dir().unwrap().to_str().unwrap());
    }
    fn default_host() -> String {
        String::from("https://demenses.net")
    }
    pub fn default_screen() -> String {
        String::new()
    }
    fn default_desc() -> String {
        String::from("A raven theme.")
    }
    //Structure for holding theme info, stored in theme.json
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ThemeStore {
        pub name: String,
        pub options: Vec<String>,
        pub enabled: Vec<String>,
        #[serde(default = "default_screen")]
        pub screenshot: String,
        #[serde(default = "default_desc")]
        pub description: String,
    }
    //Structure that holds all methods and data for individual themes.
    pub struct Theme {
        pub name: String,
        pub options: Vec<String>,
        pub monitor: i32,
        pub enabled: Vec<String>,
        pub order: Vec<String>,
    }
    //Config structure for holding all main config options
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub monitors: i32,
        pub polybar: Vec<String>,
        pub menu_command: String,
        pub last: String,
        pub editing: String,
        #[serde(default = "default_host")]
        pub host: String,
    }


    impl Config {
        pub fn default() -> Config {
            Config {
                monitors: 1,
                polybar: vec!["main".to_string(), "other".to_string()],
                menu_command: "rofi -theme sidebar -mesg 'raven:' -p '> ' -dmenu".to_string(),
                last: "".to_string(),
                editing: "".to_string(),
                host: "https://demenses.net".to_string(),
            }
        }
    }
    // Methods for a loaded theme
    impl Theme {
        //Iterates through options and loads them with submethods
        pub fn load_all(&self) {
            let opt = &self.options;
            let mut i = 1;
            let len = opt.len();
            while i <= len {
                let ref option = opt[len - i];
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
                    "rofi" => self.load_rofi(),
                    "ranger" => self.load_ranger(),
                    "lemonbar" => self.load_lemon(),
                    "openbox" => self.load_openbox(),
                    "dunst" => self.load_dunst(),
                    "|" => {}
                    _ => println!("Unknown option"),
                };
                if !option.contains("|") {
                    println!("Loaded option {}", option);
                }
                i += 1;

            }
            println!("Loaded all options for theme {}", self.name);

        }
        pub fn load_rofi(&self) {
            if fs::metadata(get_home() + "/.config/rofi").is_err() {
                fs::create_dir(get_home() + "/.config/rofi").unwrap();
            }
            fs::copy(
                get_home() + "/.config/raven/themes/" + &self.name + "/rofi",
                get_home() + "/.config/rofi/theme.rasi",
            ).expect("Couldn't copy rofi theme");
        }
        pub fn load_pywal(&self) {
            let arg = get_home() + "/.config/raven/themes/" + &self.name + "/pywal";
            Command::new("wal")
                .arg("-n")
                .arg("-i")
                .arg(arg)
                .output()
                .expect("Couldn't run pywal");
        }
        pub fn load_script(&self) {
            Command::new("sh")
                .arg("-c")
                .arg(
                    get_home() + "/.config/raven/themes/" + &self.name + "/script",
                )
                .output()
                .expect("Couldn't run custom script");
        }

        pub fn load_openbox(&self) {
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
        pub fn load_ranger(&self) {
            fs::copy(
                get_home() + "/.config/raven/themes/" + &self.name + "/ranger",
                get_home() + "/.config/ranger/rc.conf",
            ).expect("Couldn't overwrite ranger config");
        }

        pub fn load_dunst(&self) {
            let mut config = String::new();
            if fs::metadata(get_home() + "/.config/raven/base_dunst").is_ok() {
                fs::File::open(get_home() + "/.config/raven/base_dunst")
                    .unwrap()
                    .read_to_string(&mut config)
                    .unwrap();
            }
            let mut app = String::new();
            fs::File::open(
                get_home() + "/.config/raven/themes/" + &self.name + "/dunst",
            ).unwrap()
                .read_to_string(&mut app)
                .unwrap();
            config.push_str(&app);
            fs::remove_file(get_home() + "/.config/dunst/dunstrc").unwrap();
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(get_home() + "/.config/dunst/dunstrc")
                .expect("Couldn't open dunstrc")
                .write_all(config.as_bytes())
                .unwrap();
            Command::new("dunst").spawn().expect("Failed to run dunst");
        }

        pub fn load_ncm(&self) {
            if fs::metadata(get_home() + "/.config/ncmpcpp").is_ok() {
                fs::copy(
                    get_home() + "/.config/raven/themes/" + &self.name + "/ncmpcpp",
                    get_home() + "/.config/ncmpcpp/config",
                ).expect("Couldn't overwrite ncmpcpp config");
            } else if fs::metadata(get_home() + "/.ncmpcpp").is_ok() {
                fs::copy(
                    get_home() + "/.config/raven/themes/" + &self.name + "/ncmpcpp",
                    get_home() + "/.ncmpcpp/config",
                ).expect("Couldn't overwrite ncmpcpp config");
            } else {
                println!(
                    "Couldn't detect a ncmpcpp config directory in ~/.config/ncmppcp or ~/.ncmpcpp."
                );
            }

        }
        pub fn load_bspwm(&self) {
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
                .mode(0o744)
                .open(get_home() + "/.config/bspwm/bspwmrc")
                .expect("Couldn't open bspwmrc file")
                .write_all(config.as_bytes())
                .unwrap();
            Command::new("sh")
                .arg("-c")
                .arg(get_home() + "/.config/bspwm/bspwmrc")
                .output()
                .expect("Couldn't reload bspwm");
        }
        pub fn load_i3(&self, isw: bool) {
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
        pub fn load_termite(&self) {
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
        pub fn load_poly(&self, monitor: i32) {
            for number in 0..monitor {
                Command::new("sh")
                    .arg("-c")
                    .arg(
                        String::from("polybar --config=") + &get_home() +
                            "/.config/raven/themes/" + &self.name +
                            "/poly " + &self.order[number as usize] +
                            " > /dev/null 2> /dev/null",
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
    //Checks to see if base config/directories need to be initialized
    pub fn check_init() -> bool {
        if fs::metadata(get_home() + "/.config/raven").is_err() ||
            fs::metadata(get_home() + "/.config/raven/config.json").is_err() ||
            fs::metadata(get_home() + "/.config/raven/themes").is_err()
        {
            true
        } else {
            false
        }
    }
    //Check to see if there are themes still using the old format
    pub fn check_themes() {
        let entries = get_themes();
        for entry in entries {
            if fs::metadata(get_home() + "/.config/raven/themes/" + &entry + "/theme").is_ok() {
                convert_theme(&entry);
            }
        }
    }
    pub fn init() {
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
    //Start ravend
    pub fn start_daemon() {
        Command::new("sh").arg("-c").arg("ravend").spawn().expect(
            "Couldn't start daemon.",
        );
        println!("Started cycle daemon.");

    }
    pub fn stop_daemon() {
        Command::new("pkill")
            .arg("-SIGKILL")
            .arg("ravend")
            .output()
            .expect("Couldn't stop daemon.");
        println!("Stopped cycle daemon.");
    }
    pub fn check_daemon() -> bool {
        let out = Command::new("ps").arg("aux").output().expect(
            "Couldn't find daemon",
        );
        let form_out = String::from_utf8_lossy(&out.stdout);
        let line_num = form_out.lines().filter(|x| x.contains("ravend")).count();
        if line_num > 0 { true } else { false }
    }
    pub fn edit(theme_name: &str) {
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
    pub fn clear_prev() {
        Command::new("pkill").arg("polybar").output().unwrap();
        Command::new("pkill").arg("lemonbar").output().unwrap();
        Command::new("pkill").arg("dunst").output().unwrap();
    }
    pub fn del_theme(theme_name: &str) {
        fs::remove_dir_all(get_home() + "/.config/raven/themes/" + &theme_name)
            .expect("Couldn't delete theme");;
    }
    pub fn refresh_theme(last: String) {
        //Load last loaded theme
        if last.chars().count() > 0 {
            run_theme(load_theme(last.trim()).unwrap());
        } else {

            println!("No last theme saved. Cannot refresh.");
        }
    }
    pub fn proc_path(path: DirEntry) -> String {
        //Converts DirEntry into a fully processed file/directory name
        let base = path.file_name().into_string().unwrap();
        return base;
    }
    pub fn new_theme(theme_name: &str) {
        //Create new theme directory and 'theme' file
        let res = fs::create_dir(get_home() + "/.config/raven/themes/" + &theme_name);
        if res.is_ok() {
            res.unwrap();
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(
                    get_home() + "/.config/raven/themes/" + &theme_name + "/theme.json",
                )
                .expect("can open");
            let stdef = ThemeStore {
                name: String::from(theme_name),
                options: vec![],
                enabled: vec![],
                screenshot: default_screen(),
                description: default_desc(),
            };
            let st = serde_json::to_string(&stdef).unwrap();
            file.write_all(st.as_bytes()).unwrap();
            edit(&theme_name);
        } else {
            println!("Theme {} already exists", &theme_name);
        }
    }


    pub fn add_to_theme(theme_name: &str, option: &str, path: &str) {
        //Add an option to a theme
        let cur_theme = load_theme(theme_name).unwrap();
        let cur_st = load_store(String::from(theme_name));
        let mut new_themes = ThemeStore {
            name: theme_name.to_string(),
            options: cur_theme.options,
            enabled: cur_theme.enabled,
            screenshot: cur_st.screenshot,
            description: cur_st.description,
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
    pub fn rm_from_theme(theme_name: &str, option: &str) {
        //Remove an option from a theme
        let cur_theme = load_theme(theme_name).unwrap();
        let cur_st = load_store(String::from(theme_name));
        let mut new_themes = ThemeStore {
            name: theme_name.to_string(),
            options: cur_theme.options,
            enabled: cur_theme.enabled,
            screenshot: cur_st.screenshot,
            description: cur_st.description,
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
    pub fn run_theme(new_theme: Theme) {
        //Run/refresh a loaded Theme
        new_theme.load_all();
        let mut conf = get_config();
        conf.last = new_theme.name;
        up_config(conf);
    }
    pub fn up_config(conf: Config) {
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
    pub fn up_theme(theme: ThemeStore) {
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

    pub fn convert_theme(theme_name: &str) {
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
            screenshot: default_screen(),
            description: default_desc(),
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
    pub fn load_store(theme: String) -> ThemeStore {
        let mut st = String::new();
        fs::File::open(
            get_home() + "/.config/raven/themes/" + &theme + "/theme.json",
        ).unwrap()
            .read_to_string(&mut st)
            .unwrap();
        let thinf: ThemeStore = serde_json::from_str(&st).unwrap();
        thinf
    }
    pub fn load_theme(theme_name: &str) -> Result<Theme, &'static str> {
        //Load in data for and run loading methods for a specific theme
        let conf = get_config();
        let ent_res = fs::read_dir(get_home() + "/.config/raven/themes/" + &theme_name);
        if ent_res.is_ok() {
            println!("Found theme {}", theme_name);
            if fs::metadata(
                get_home() + "/.config/raven/themes/" + &theme_name + "/theme.json",
            ).is_ok()
            {
                let theme_info = load_store(String::from(theme_name));
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

    pub fn get_config() -> Config {
        //Retrieve config settings from file
        let mut conf = String::new();
        fs::File::open(get_home() + "/.config/raven/config.json")
            .expect("Couldn't read config")
            .read_to_string(&mut conf)
            .unwrap();
        let config: Config = serde_json::from_str(&conf).expect("Couldn't read config file");
        config
    }
    pub fn get_themes() -> Vec<String> {
        let mut entries = fs::read_dir(get_home() + "/.config/raven/themes")
            .expect("Couldn't read themes")
            .collect::<Vec<io::Result<DirEntry>>>()
            .into_iter()
            .map(|x| proc_path(x.unwrap()))
            .collect::<Vec<String>>();
        return entries;
    }
}
