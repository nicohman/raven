use std::fs;
extern crate time;
use std::time::Duration;
use std::io;
use std::io::Read;
use std::fs::DirEntry;
use std::env;

use std::thread;
use std::process::Command;
fn main () {
    if fs::metadata(get_home()+"/.config/raven/time").is_err() {
        println!("There is no time configured for ravend.");
    } else {
        let mut time = String::new();
        fs::File::open(get_home()+"/.config/raven/time").unwrap().read_to_string(&mut time).unwrap();
        time = String::from(time.trim());
        let time_r = time.parse::<i32>();
        if time_r.is_ok() {
            let time_t = time_r.unwrap();
            let themes = fs::read_dir(get_home()+"/.config/raven/themes").expect("Couldn't read themes").collect::<Vec<io::Result<DirEntry>>>().into_iter().map(|x| proc_path(x.unwrap())).collect::<Vec<String>>();
            start_cycle(themes, time_t);
        } else {
            println!("Time file does not contain a number in seconds. {}", time);
        }
    }
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn start_cycle(entries : Vec<String> , time: i32) {
    let mut index = 0;
    loop {
    let len  = entries.len();
    if index >= len {
        index = 0;
    }
    Command::new("sh").arg("-c").arg(String::from("raven load ")+&entries[index]+" &").spawn().expect("Failed to swap.");
    thread::sleep(Duration::from_secs(time as u64));
    println!("Changing theme!");
    index = index +1;
    }
    }
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
