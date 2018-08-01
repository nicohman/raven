pub mod ravens {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Read;
    use std::env;
    use serde_json;
    use std::io::Write;
    use tar::{Archive, Builder};
    use reqwest;
    fn get_home() -> String {
        return String::from(env::home_dir().unwrap().to_str().unwrap());
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserInfo {
        name: String,
        token: String,
    }
    pub fn load_info() -> Result<UserInfo, String> {
        if fs::metadata(get_home() + "/.config/raven/ravenserver.json").is_ok() {
            let mut info = String::new();
            File::open(get_home() + "/.config/raven/ravenserver.json")
                .expect("Couldn't read user info")
                .read_to_string(&mut info)
                .unwrap();
            let un = serde_json::from_str(&info);
            if un.is_ok() {
                Ok(un.unwrap())
            } else {
                Err("User info file in incorrect state".to_string())
            }
        } else {
            Err("Not logged in".to_string())
        }
    }
    pub fn export(theme_name: &str) {
        if fs::metadata(get_home() + "/.config/raven/themes/" + theme_name).is_ok() {
            let tb = File::create(theme_name.to_string() + ".tar").unwrap();
            let mut b = Builder::new(tb);
            b.append_dir_all(
                theme_name.to_string(),
                get_home() + "/.config/raven/themes/" + theme_name,
            ).expect("Couldn't add theme to archive");
            b.into_inner().expect("Couldn't write tar archive");
            println!("Wrote theme to {}.tar", theme_name)
        } else {
            println!("Theme does not exist");
        }
    }
    pub fn import(file_name: &str) {
        if fs::metadata(file_name).is_ok() {
            let mut arch = Archive::new(File::open(file_name).unwrap());
            arch.unpack(get_home() + "/.config/raven/themes/").expect(
                "Couldn't unpack theme archive",
            );
            println!("Imported theme.");
        }
    }
    fn up_info(inf: UserInfo) {
        let winfpath = get_home() + "/.config/raven/~ravenserver.json";
        let infpath = get_home() + "/.config/raven/ravenserver.json";
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&winfpath)
            .expect("Couldn't open user info file")
            .write_all(serde_json::to_string(&inf).unwrap().as_bytes())
            .expect("Couldn't write to user info file");
        fs::copy(&winfpath, &infpath).unwrap();
        fs::remove_file(&winfpath).unwrap();
    }
    pub fn create_user(name: String, pass: String) {
        let client = reqwest::Client::new();
        let res = client
            .post(
                &("https://demenses.net/themes/user/create?name=".to_string() + &name + "&pass=" +
                    &pass),
            )
            .send();
        if res.is_ok() {
            let res = res.unwrap();
            if res.status().is_success() {
                println!(
                    "Successfully created user. Now, sign in with `raven login [name] [password]`"
                );
            } else {
                if res.status() == reqwest::StatusCode::Forbidden {
                    println!("User already created. Pick a different name!");
                } else {
                    println!("Server error. Code {:?}", res.status());
                }
            }
        } else {
            println!("Something went wrong with creating a user. Error message:");
            println!("{:?}", res);
        }
    }
    pub fn upload_theme(name: String) {
        let info = load_info().unwrap();
        if fs::metadata(get_home() + "/.config/raven/themes/" + &name).is_ok() {
            export(&name);
            if fs::metadata(name.clone() + ".tar").is_ok() {
                let form = reqwest::multipart::Form::new()
                    .file("fileupload", name.clone() + ".tar")
                    .unwrap();
                let res = reqwest::Client::new()
                    .post(
                        &("https://demenses.net/themes/upload?name=".to_string() + &name +
                              "&token=" +
                              &info.token),
                    )
                    .multipart(form)
                    .send();

                if res.is_ok() {
                    let res = res.unwrap();
                    if res.status().is_success() {
                        if res.status() == reqwest::StatusCode::Created {
                            println!("Theme successfully uploaded.");
                        } else {
                            println!("Theme successfully updated.");
                        }
                        fs::remove_file(name + ".tar").unwrap();
                    } else {
                        if res.status() == reqwest::StatusCode::Forbidden {
                            println!("That theme already exists, and you are not its owner.");
                        } else {
                            println!("Server error. Code {:?}", res.status());
                        }

                    }
                } else {
                    println!("Something went wrong with uploading the theme. Error message:");
                    println!("{:?}", res);

                }
            } else {
                println!(
                    "Something has gone wrong. Check if the theme file was written to current directory."
                );
            }
        } else {
            println!("That theme does not exist");
            }
    }
    pub fn unpublish_theme(name:String) {
        let info = load_info().unwrap();
        let client = reqwest::Client::new();
        let res = client.post(&("https://demenses.net/themes/delete/".to_string()+&name+"?token="+&info.token)).send();
        if res.is_ok() {
            let mut res = res.unwrap();
            if res.status().is_success() {
                println!("Successfully unpublished theme");
            } else {
                if res.status() == reqwest::StatusCode::NotFound {
                    println!("Can't unpublish a nonexistent theme");
                } else if res.status() == reqwest::StatusCode::Forbidden {
                    println!("Can't unpublish a theme that isn't yours");
                } else if res.status() == reqwest::StatusCode::Unauthorized {
                    println!("Did not provide a valid login token");
                } else {
                    println!("Server error. Code {:?}", res.status());
                
                }
            }
        } else {
            println!("Something went wrong with unpublishing the theme. Error message: ");
            println!("{:?}", res);
        }
    }
    pub fn download_theme(name: String) {
        let client = reqwest::Client::new();
        let res = client
            .get(&("https://demenses.net/themes/repo/".to_string() + &name))
            .send();
        if res.is_ok() {
            let mut res = res.unwrap();
            if res.status().is_success() {
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(name.clone() + ".tar")
                    .expect("Couldn't write theme file");
                res.copy_to(&mut file).expect("Couldn't pipe to archive");
                println!("Downloaded theme");
                import(&(name.clone()+".tar"));
                println!("Imported theme. Removing archive.");
                fs::remove_file(name.clone()+".tar").unwrap();
            } else {
                if res.status() == reqwest::StatusCode::NotFound {
                    println!("Theme has not been uploaded");
                } else {
                    println!("Server error. Code {:?}", res.status());
                }
            }
        } else {
            println!("Something went wrong with downloading the theme. Error message:");
            println!("{:?}", res);
        }
    }
    pub fn login_user(name: String, pass: String) {
        let client = reqwest::Client::new();
        let res =
            client
                .get(
                    &("https://demenses.net/themes/user/login?name=".to_string() + &name +
                          "&pass=" +
                          &pass),
                )
                .send();
        if res.is_ok() {
            let mut res = res.unwrap();
            if res.status().is_success() {
                println!("Successfully signed in. Writing login info to disk.");
                let info = res.json().unwrap();
                up_info(info);
            } else {
                if res.status() == reqwest::StatusCode::Forbidden {
                    println!("Wrong login info. Try again!");
                } else {
                    println!("Server error. Code {:?}", res.status());
                }
            }
        } else {
            println!("Something went wrong with logging in. Error message:");
            println!("{:?}", res);
        }
    }
}
