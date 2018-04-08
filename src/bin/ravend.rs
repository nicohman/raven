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
        let timeR = time.parse::<i32>();
        if timeR.is_ok() {
            let mut timeT = timeR.unwrap();
            println!("{}", timeT);
            let mut themes = fs::read_dir(get_home()+"/.config/raven/themes").expect("Couldn't read themes").collect::<Vec<io::Result<DirEntry>>>().into_iter().map(|x| proc_path(x.unwrap())).collect::<Vec<String>>();
            let mut ind = 0;
            start_cycle(themes, timeT);
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
    let mut len  = entries.len();
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
