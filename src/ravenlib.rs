extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate multipart;
extern crate reqwest;
extern crate tar;
pub mod ravenserver;
use std::fs::DirEntry;
/// Module for theme manipulation
pub mod themes;
/// Config module
pub mod config {
    use crate::themes::*;
    use dirs::home_dir;
    use std::{fs, fs::OpenOptions, io::Read, io::Write};
    /// Returns home directory as string
    pub fn get_home() -> String {
        return String::from(home_dir().unwrap().to_str().unwrap());
    }
    /// Default ravenserver host
    pub fn default_host() -> String {
        String::from("https://demenses.net")
    }
    /// Default screenshot url
    pub fn default_screen() -> String {
        String::new()
    }
    /// Default raven theme description
    pub fn default_desc() -> String {
        String::from("A raven theme.")
    }
    /// Config structure for holding all main config options
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
        /// Default method for config file
        pub fn default() -> Config {
            Config {
                monitors: 1,
                polybar: vec!["main".to_string(), "other".to_string()],
                menu_command: "rofi -theme sidebar -mesg 'raven:' -p '> ' -dmenu".to_string(),
                last: "".to_string(),
                editing: "".to_string(),
                host: default_host(),
            }
        }
    }
    /// Check to see if there are themes still using the old format
    pub fn check_themes() {
        let entries = get_themes();
        for entry in entries {
            if fs::metadata(get_home() + "/.config/raven/themes/" + &entry + "/theme").is_ok() {
                convert_theme(&entry);
            }
        }
    }
    /// Create base raven directories and config file(s)
    pub fn init() {
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
    /// Checks to see if base config/directories need to be initialized
    pub fn check_init() -> bool {
        fs::metadata(get_home() + "/.config/raven").is_err()
            || fs::metadata(get_home() + "/.config/raven/config.json").is_err()
            || fs::metadata(get_home() + "/.config/raven/themes").is_err()
    }
    /// Updates the written config with a new config
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
        )
        .unwrap();
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
        fs::File::open(get_home() + "/.config/raven/themes/" + theme_name + "/theme")
            .expect("Couldn't read theme")
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
        fs::remove_file(get_home() + "/.config/raven/themes/" + theme_name + "/theme").unwrap();
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/raven/themes/" + theme_name + "/theme.json")
            .expect("Can't open theme.json")
            .write_all(serde_json::to_string(&themes).unwrap().as_bytes())
            .unwrap();
    }
    pub fn load_store(theme: String) -> ThemeStore {
        let mut st = String::new();
        fs::File::open(get_home() + "/.config/raven/themes/" + &theme + "/theme.json")
            .unwrap()
            .read_to_string(&mut st)
            .unwrap();
        serde_json::from_str(&st).unwrap()
    }
    pub fn load_theme(theme_name: &str) -> Result<Theme, &'static str> {
        //Load in data for and run loading methods for a specific theme
        let conf = get_config();
        let ent_res = fs::read_dir(get_home() + "/.config/raven/themes/" + &theme_name);
        if ent_res.is_ok() {
            println!("Found theme {}", theme_name);
            if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name + "/theme.json")
                .is_ok()
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
            Err("Theme does not exist")
        }
    }

    pub fn get_config() -> Config {
        //Retrieve config settings from file
        let mut conf = String::new();
        fs::File::open(get_home() + "/.config/raven/config.json")
            .expect("Couldn't read config")
            .read_to_string(&mut conf)
            .unwrap();
        serde_json::from_str(&conf).expect("Couldn't read config file")
    }
}
/// Ravend control
pub mod daemon {
    use std::process::Command;
    /// Starts ravend
    pub fn start_daemon() {
        Command::new("sh")
            .arg("-c")
            .arg("ravend")
            .spawn()
            .expect("Couldn't start daemon.");
        println!("Started cycle daemon.");
    }
    /// Stops ravend
    pub fn stop_daemon() {
        Command::new("pkill")
            .arg("-SIGKILL")
            .arg("ravend")
            .output()
            .expect("Couldn't stop daemon.");
        println!("Stopped cycle daemon.");
    }
    /// Checks if the ravend daemon is running
    pub fn check_daemon() -> bool {
        let out = Command::new("ps")
            .arg("aux")
            .output()
            .expect("Couldn't find daemon");
        let form_out = String::from_utf8_lossy(&out.stdout);
        let line_num = form_out.lines().filter(|x| x.contains("ravend")).count();
        line_num > 0
    }

}

/// Converts DirEntry into a fully processed file/directory name
pub fn proc_path(path: DirEntry) -> String {
    path.file_name().into_string().unwrap()
}
