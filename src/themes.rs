use crate::config::*;
use proc_path;
use serde_json::value::{Map, Value};
use std::{
    env, fs, fs::DirEntry, fs::OpenOptions, io, io::Read, io::Write, os::unix::fs::OpenOptionsExt,
    process::Command,
};
/// Structure for holding theme info, stored in theme.json
#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeStore {
    pub name: String,
    pub options: Vec<String>,
    pub enabled: Vec<String>,
    #[serde(default = "default_screen")]
    pub screenshot: String,
    #[serde(default = "default_desc")]
    pub description: String,
    #[serde(default)]
    pub kv: Map<String, Value>,
}
/// Structure that holds all methods and data for individual themes.
#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub options: Vec<String>,
    pub monitor: i32,
    pub enabled: Vec<String>,
    pub order: Vec<String>,
    pub kv: Map<String, Value>,
}

/// Methods for a loaded theme
impl Theme {
    /// Loads options held within theme.json key-value storage
    pub fn load_kv(&self) {
        for (k, v) in &self.kv {
            self.load_k(k.as_str(), v.as_str().unwrap());
        }
    }
    /// Loads a single key option
    pub fn load_k<N>(&self, k: N, v: N)
    where
        N: Into<String>,
    {
        let (k, v) = (k.into(), v.into());
        match k.as_str() {
            "st_tmtheme" => self.load_sublt("st_tmtheme", v.as_str()),
            "st_scs" => self.load_sublt("st_scs", v.as_str()),
            "st_subltheme" => self.load_sublt("st_subltheme", v.as_str()),
            "vscode" => self.load_vscode(v.as_str()),
            _ => println!("Unrecognized key {}", k),
        }
        println!("Loaded key option {}", k);
    }
    /// Converts old single-string file options into key-value storage
    pub fn convert_single<N>(&self, name: N)
    where
        N: Into<String>,
    {
        let key = name.into();
        let mut value = String::new();
        fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/" + &key)
            .expect("Couldn't open file")
            .read_to_string(&mut value)
            .unwrap();
        let mut store = load_store(self.name.clone());
        store.kv.insert(key.clone(),serde_json::Value::String(value.clone().trim().to_string()));
        store.options = store.options.iter().filter(|x| x.as_str() != key.as_str()).map(|x|x.to_owned()).collect();
        up_theme(store);
        println!("Converted option {} to new key-value system", key);
        self.load_k(key, value);
    }
    /// Iterates through options and loads them with submethods
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
                "st_tmtheme" => self.convert_single("st_tmtheme"),
                "st_scs" => self.convert_single("st_scs"),
                "st_subltheme" => self.convert_single("st_subltheme"),
                "vscode" => self.convert_single("vscode"),
                "|" => {}
                _ => println!("Unknown option"),
            };
            if !option.contains("|") {
                println!("Loaded option {}", option);
            }
            i += 1;
        }
        self.load_kv();
        println!("Loaded all options for theme {}", self.name);
    }
    /// Edits the value of a key in hjson files
    fn edit_hjson<N, S, T>(&self, file: N, pat: S, value: T)
    where
        N: Into<String>,
        S: Into<String>,
        T: Into<String>,
    {
        let file = &file.into();
        let pat = &pat.into();
        let value = &value.into();
        let mut finals = String::new();
        if fs::metadata(file).is_ok() {
            let mut pre = String::new();
            fs::File::open(file)
                .expect("Couldn't open hjson file")
                .read_to_string(&mut pre)
                .unwrap();
            let mut patfound = false;
            for line in pre.lines() {
                if line.contains(pat) {
                    patfound = true;
                    if line.ends_with(",") {
                        finals = finals + "\n" + "    " + pat + "\"" + &value + "\","
                    } else {
                        finals = finals + "\n" + "    " + pat + "\"" + &value + "\""
                    }
                } else if line.ends_with("}") && !patfound {
                    finals =
                        finals + "," + "\n" + "    " + pat + "\"" + &value + "\"" + "\n" + line;
                } else {
                    finals = finals + "\n" + line;
                }
            }
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                .expect("Couldn't open hjson file")
                .write_all(finals.trim().as_bytes())
                .unwrap();
        } else {
            finals = finals + "{\n    " + pat + "\"" + &value + "\"\n}";
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(file)
                .expect("Couldn't open hjson file")
                .write_all(finals.as_bytes())
                .unwrap();
        }
    }
    pub fn load_rofi(&self) {
        if fs::metadata(get_home() + "/.config/rofi").is_err() {
            fs::create_dir(get_home() + "/.config/rofi").unwrap();
        }
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/rofi",
            get_home() + "/.config/rofi/theme.rasi",
        )
        .expect("Couldn't copy rofi theme");
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
            .arg(get_home() + "/.config/raven/themes/" + &self.name + "/script")
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
        fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/openbox")
            .unwrap()
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
        )
        .expect("Couldn't overwrite ranger config");
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
        fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/dunst")
            .unwrap()
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
    pub fn load_vscode<N>(&self, value: N)
    where
        N: Into<String>,
    {
        let path1 = get_home() + "/.config/Code/User";
        let path2 = get_home() + "/.config/Code - OSS/User";
        if fs::metadata(&path1).is_err() && fs::metadata(&path2).is_err() {
            println!(
                "Couldn't find neither .config/Code nor .config/Code - OSS. Do you have VSCode installed? \
                Skipping."
            );
            return;
        }
        let pattern = "\"workbench.colorTheme\": ";
        let value = value.into();
        if fs::metadata(&path1).is_ok() {
            self.edit_hjson(path1 + "/settings.json", pattern, value.as_str())
        }
        if fs::metadata(&path2).is_ok() {
            self.edit_hjson(path2 + "/settings.json", pattern, value)
        }
    }
    pub fn load_sublt<N>(&self, stype: N, value: N)
    where
        N: Into<String>,
    {
        let stype = &stype.into();
        let path = get_home() + "/.config/sublime-text-3/Packages/User";
        if fs::metadata(&path).is_err() {
            println!(
                "Couldn't find {}. Do you have sublime text 3 installed? \
                 Skipping.",
                &path
            );
            return;
        }

        let mut value = value.into();
        if value.starts_with("sublt/") {
            value = value.trim_start_matches("sublt/").to_string();
            fs::copy(
                get_home() + "/.config/raven/themes/" + &self.name + "/sublt/" + &value,
                path.clone() + "/" + &value
            )
            .expect("Couldn't overwrite sublt theme");
        }

        let mut pattern = "";
        if stype == "st_tmtheme" || stype == "st_scs" {
            pattern = "\"color_scheme\": ";
        } else if stype == "st_subltheme" {
            pattern = "\"theme\": ";
        }
        self.edit_hjson(path + "/Preferences.sublime-settings", pattern, value)
    }

    pub fn load_ncm(&self) {
        if fs::metadata(get_home() + "/.config/ncmpcpp").is_ok() {
            fs::copy(
                get_home() + "/.config/raven/themes/" + &self.name + "/ncmpcpp",
                get_home() + "/.config/ncmpcpp/config",
            )
            .expect("Couldn't overwrite ncmpcpp config");
        } else if fs::metadata(get_home() + "/.ncmpcpp").is_ok() {
            fs::copy(
                get_home() + "/.config/raven/themes/" + &self.name + "/ncmpcpp",
                get_home() + "/.ncmpcpp/config",
            )
            .expect("Couldn't overwrite ncmpcpp config");
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
        fs::File::open(get_home() + "/.config/raven/themes/" + &self.name + "/bspwm")
            .unwrap()
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
        if fs::metadata(get_home() + "/.config/i3").is_err() {
            fs::create_dir(get_home() + "/.config/i3").expect("Couldn't create i3 config dir");
        }
        if fs::metadata(get_home() + "/.config/i3/config").is_ok() {
            fs::remove_file(get_home() + "/.config/i3/config")
                .expect("Couldn't remove previous i3 config");
        }
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/i3/config")
            .expect("Couldn't open i3 file")
            .write_all(config.as_bytes())
            .unwrap();
        Command::new("i3-msg")
            .arg("reload")
            .output()
            .expect("Couldn't reload i3");
    }
    pub fn load_termite(&self) {
        fs::copy(
            get_home() + "/.config/raven/themes/" + &self.name + "/termite",
            get_home() + "/.config/termite/config",
        )
        .expect("Couldn't overwrite termite config");
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
                    String::from("polybar --config=")
                        + &get_home()
                        + "/.config/raven/themes/"
                        + &self.name
                        + "/poly "
                        + &self.order[number as usize]
                        + " > /dev/null 2> /dev/null",
                )
                .spawn()
                .expect("Failed to run polybar");
        }
    }
    fn load_lemon(&self) {
        Command::new("sh")
            .arg(get_home() + "/.config/raven/themes/" + &self.name + "/lemonbar")
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
        xres.arg(get_home() + "/.config/raven/themes/" + &self.name + "/" + &name)
            .output()
            .expect("Could not run xrdb");
    }
}

/// Changes the theme that is currently being edited
pub fn edit<N>(theme_name: N)
where
    N: Into<String>,
{
    let theme_name = theme_name.into();
    if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name).is_ok() {
        let mut conf = get_config();
        conf.editing = theme_name.to_string();
        up_config(conf);
        println!("You are now editing the theme {}", &theme_name);
    } else {
        println!("That theme does not exist");
    }
}
/// Clears possible remnants of old themes
pub fn clear_prev() {
    Command::new("pkill").arg("polybar").output().unwrap();
    Command::new("pkill").arg("lemonbar").output().unwrap();
    Command::new("pkill").arg("dunst").output().unwrap();
}
/// Deletes theme from registry
pub fn del_theme<N>(theme_name: N)
where
    N: Into<String>,
{
    fs::remove_dir_all(get_home() + "/.config/raven/themes/" + &theme_name.into())
        .expect("Couldn't delete theme");;
}
/// Loads last loaded theme from string of last theme's name
pub fn refresh_theme<N>(last: N)
where
    N: Into<String>,
{
    let last = last.into();
    if last.chars().count() > 0 {
        run_theme(load_theme(last.trim()).unwrap());
    } else {
        println!("No last theme saved. Cannot refresh.");
    }
}
/// Create new theme directory and 'theme' file
pub fn new_theme<N>(theme_name: N)
where
    N: Into<String>,
{
    let theme_name = theme_name.into();
    let res = fs::create_dir(get_home() + "/.config/raven/themes/" + &theme_name);
    if res.is_ok() {
        res.unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/raven/themes/" + &theme_name + "/theme.json")
            .expect("can open");
        let stdef = ThemeStore {
            name: theme_name.clone(),
            options: vec![],
            enabled: vec![],
            screenshot: default_screen(),
            description: default_desc(),
            kv: Map::new(),
        };
        let st = serde_json::to_string(&stdef).unwrap();
        file.write_all(st.as_bytes()).unwrap();
        edit(theme_name);
    } else {
        println!("Theme {} already exists", &theme_name);
    }
}
/// Add an option to a theme
pub fn add_to_theme<N>(theme_name: N, option: N, path: N)
where
    N: Into<String>,
{
    let (theme_name, option, path) = (theme_name.into(), option.into(), path.into());
    let cur_theme = load_theme(theme_name.as_str()).unwrap();
    let cur_st = load_store(theme_name.as_str());
    let mut new_themes = ThemeStore {
        name: theme_name.clone(),
        options: cur_theme.options,
        enabled: cur_theme.enabled,
        screenshot: cur_st.screenshot,
        description: cur_st.description,
        kv: Map::new(),
    };
    let mut already_used = false;
    for opt in &new_themes.options {
        if opt == &option {
            already_used = true;
        }
    }
    if !already_used {
        new_themes.options.push(option.clone());
        up_theme(new_themes);
    }
    let mut totpath = env::current_dir().unwrap();
    totpath.push(path);
    fs::copy(
        totpath,
        get_home() + "/.config/raven/themes/" + &theme_name + "/" + &option,
    )
    .expect("Couldn't copy config in");
}
/// Remove an option from a theme
pub fn rm_from_theme<N>(theme_name: N, option: N)
where
    N: Into<String>,
{
    let (theme_name, option) = (theme_name.into(), option.into());
    let cur_theme = load_theme(theme_name.as_str()).unwrap();
    let cur_st = load_store(theme_name.as_str());
    let mut new_themes = ThemeStore {
        name: theme_name,
        options: cur_theme.options,
        enabled: cur_theme.enabled,
        screenshot: cur_st.screenshot,
        description: cur_st.description,
        kv: Map::new(),
    };
    let mut found = false;
    let mut i = 0;
    while i < new_themes.options.len() {
        if &new_themes.options[i] == &option {
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
/// Run/refresh a loaded Theme
pub fn run_theme(new_theme: Theme) {
    new_theme.load_all();
    // Updates the 'last loaded theme' information for later use by raven refresh
    let mut conf = get_config();
    conf.last = new_theme.name;
    up_config(conf);
}
/// Get all themes
pub fn get_themes() -> Vec<String> {
    fs::read_dir(get_home() + "/.config/raven/themes")
        .expect("Couldn't read themes")
        .collect::<Vec<io::Result<DirEntry>>>()
        .into_iter()
        .map(|x| proc_path(x.unwrap()))
        .collect::<Vec<String>>()
}
/// Changes a key-value option
pub fn key_value<N, S, T>(key: N, value: S, theme: T) where N : Into<String>, S: Into<String>, T: Into<String> {
    let mut store = load_store(theme.into());
    store.kv.insert(key.into(), serde_json::Value::String(value.into()));
    up_theme(store);
}
