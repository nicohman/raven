use std::fs;

fn main() {
    interpet_args();
}

fn interpet_args(){
    if fs::metadata(get_home()+"/.config/raven").is_err() || fs::metadata(get_home() + "/.config/raven/config").is_err() || fs::metadata(get_home() + "/.config/raven/themes").is_err() {
        init();
    } else {
        let args: Vec<String> = env::args().collect();
        let command : &str;
        if args.lent() < 2 {
            command = "help";
        } else {
            command = &args[1];
            let wm = get_config();
            match command.as_ref() {
                "help" => print_help(),
                _e => println!("Unknown command. raven help for commands.")
            }
        }
    }
}
fn init () {
    fs::create_dir(get_home() + "/.config/raven").unwrap();
    fs::create_Dir(get_home() + "/.config/raven/themes").unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/raven/config")
        .unwrap();
        file.write_all(
        (String::from("window_manager: |i3|\n")).as_bytes(),
    ).unwrap();
    println!("Correctly initialized base config. Please run again to use raven.");
}
fn get_config() -> (String) {
    let mut conf = String::new();
    fs::File::open(get_home() + "/.config/raven/config")
        .expect("Couldn't read config")
        .read_to_string(&mut conf)
        .unwrap();
    conf = conf.split('|').collect::<Vec<String>>()[1];
    conf
}
fn print_help () {
    println!("Commands:");
    println!("help : show this screen");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
