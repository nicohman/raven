use config::*;
use dirs::home_dir;
use reqwest;
use serde_json;
use std::{
    fs,
    fs::{File, OpenOptions},
    io,
    io::{Read, Write},
};
use tar::{Archive, Builder};
fn get_home() -> String {
    return String::from(home_dir().unwrap().to_str().unwrap());
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    name: String,
    token: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaRes {
    screen: String,
    description: String,
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
pub fn export<N>(theme_name: N, tmp: bool) -> String
where
    N: Into<String>,
{
    let theme_name = theme_name.into();
    if fs::metadata(get_home() + "/.config/raven/themes/" + &theme_name).is_ok() {
        let mut tname = String::new();
        if tmp {
            tname = tname + "/tmp/";
        }
        tname = tname + &theme_name.to_string() + ".tar";
        let tb = File::create(&tname).unwrap();
        let mut b = Builder::new(tb);
        b.append_dir_all(
            theme_name.to_string(),
            get_home() + "/.config/raven/themes/" + &theme_name,
        )
        .expect("Couldn't add theme to archive");
        b.into_inner().expect("Couldn't write tar archive");
        println!("Wrote theme to {}", tname);
        tname
    } else {
        println!("Theme does not exist");
        ::std::process::exit(64);
    }
}
pub fn import<N>(file_name: N)
where
    N: Into<String>,
{
    let fname: String = file_name.into();
    if fs::metadata(&fname).is_ok() {
        let mut arch = Archive::new(File::open(fname).unwrap());
        arch.unpack(get_home() + "/.config/raven/themes/")
            .expect("Couldn't unpack theme archive");
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
pub fn logout() {
    fs::remove_file(get_home() + "/.config/raven/ravenserver.json")
        .expect("Couldn't delete user info file");
    println!("Successfully logged you out");
}
pub fn get_host() -> String {
    let conf = get_config();
    conf.host
}
pub fn delete_user<N>(pass: N)
where
    N: Into<String>,
{
    let info = load_info().unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(
            &(get_host()
                + "/themes/users/delete/"
                + &info.name
                + "?token="
                + &info.token
                + "&pass="
                + &pass.into()),
        )
        .send();
    if res.is_ok() {
        let res = res.unwrap();
        if res.status().is_success() {
            println!("Successfully deleted user and all owned themes. Logging out");
            logout();
        } else {
            if res.status() == reqwest::StatusCode::FORBIDDEN {
                println!("You are trying to delete a user you are not. Bad!");
            } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
                println!("You're trying to delete a user w/o providing authentication credentials");
            } else if res.status() == reqwest::StatusCode::NOT_FOUND {
                println!("You're trying to delete a user that doesn't exist");
            } else {
                println!("Server error. Code {:?}", res.status());
            }
        }
    } else {
        println!("Something went wrong with deleting your user. Error message:");
        println!("{:?}", res);
    }
}
pub fn create_user<N>(name: N, pass: N, pass2: N)
where
    N: Into<String>,
{
    let (name, pass, pass2): (String, String, String) = (name.into(), pass.into(), pass2.into());
    if pass == pass2 {
        let client = reqwest::Client::new();
        let res = client
            .post(&(get_host() + "/themes/user/create?name=" + &name + "&pass=" + &pass))
            .send();
        if res.is_ok() {
            let res = res.unwrap();
            if res.status().is_success() {
                println!("Successfully created user. Sign in with `raven login [name] [password]`");
            } else {
                if res.status() == reqwest::StatusCode::FORBIDDEN {
                    println!("User already created. Pick a different name!");
                } else if res.status() == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
                    println!(
                            "Either your username or password was too long. The limit is 20 characters for username, and 100 for password."
                        );
                } else {
                    println!("Server error. Code {:?}", res.status());
                }
            }
        } else {
            println!("Something went wrong with creating a user. Error message:");
            println!("{:?}", res);
        }
    } else {
        println!("Passwords need to match");
    }
}
pub fn upload_theme<N>(name: N)
where
    N: Into<String>,
{
    let name = name.into();
    let info = load_info().unwrap();
    if fs::metadata(get_home() + "/.config/raven/themes/" + &name).is_ok() {
        let tname = export(name.as_str(), true);
        if fs::metadata(name.clone() + ".tar").is_ok() {
            let form = reqwest::multipart::Form::new()
                .file("fileupload", &tname)
                .unwrap();
            let res = reqwest::Client::new()
                .post(&(get_host() + "/themes/upload?name=" + &name + "&token=" + &info.token))
                .multipart(form)
                .send();
            if res.is_ok() {
                let res = res.unwrap();
                if res.status().is_success() {
                    if res.status() == reqwest::StatusCode::CREATED {
                        println!("Theme successfully uploaded.");
                    } else {
                        println!("Theme successfully updated.");
                    }
                    let theme_st = load_store(name.as_str());
                    if theme_st.screenshot != default_screen() {
                        pub_metadata(name.as_str(), "screen".into(), &theme_st.screenshot);
                    }
                    pub_metadata(name, "description".into(), theme_st.description);
                    fs::remove_file(tname).unwrap();
                } else {
                    if res.status() == reqwest::StatusCode::FORBIDDEN {
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
pub fn get_metadata<N>(name: N) -> Result<MetaRes, String>
where
    N: Into<String>,
{
    let client = reqwest::Client::new();
    let res = client
        .get(&(get_host() + "/themes/meta/" + &name.into()))
        .send();
    if res.is_ok() {
        let mut res = res.unwrap();
        if res.status().is_success() {
            let meta: MetaRes = res.json().expect("Couldn't deserialize metadata responnse");
            Ok(meta)
        } else {
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                Err("Theme not found".to_string())
            } else {
                Err("Internal Server Error".to_string())
            }
        }
    } else {
        Err("Could not fetch metadata".to_string())
    }
}
pub fn pub_metadata<N>(name: N, typem: N, value: N)
where
    N: Into<String>,
{
    let info = load_info().unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(
            &(get_host()
                + "/themes/meta/"
                + &name.into()
                + "?typem="
                + &typem.into()
                + "&value="
                + &value.into()
                + "&token="
                + &info.token),
        )
        .send();
    if res.is_ok() {
        let res = res.unwrap();
        if res.status().is_success() {
            println!("Successfully updated theme metadata");
        } else {
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                println!("That theme hasn't been published");
            } else if res.status() == reqwest::StatusCode::FORBIDDEN {
                println!("Can't edit the metadata of a theme that isn't yours");
            } else if res.status() == reqwest::StatusCode::PRECONDITION_FAILED {
                println!("That isn't a valid metadata type");
            } else if res.status() == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
                println!(
                        "Your description or screenshot url was more than 200 characters long. Please shorten it."
                    );
            } else {
                println!("Server error. Code {:?}", res.status());
            }
        }
    }
}
pub fn unpublish_theme<N>(name: N)
where
    N: Into<String>,
{
    let name = name.into();
    let info = load_info().unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(&(get_host() + "/themes/delete/" + &name + "?token=" + &info.token))
        .send();
    if res.is_ok() {
        let res = res.unwrap();
        if res.status().is_success() {
            println!("Successfully unpublished theme");
        } else {
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                println!("Can't unpublish a nonexistent theme");
            } else if res.status() == reqwest::StatusCode::FORBIDDEN {
                println!("Can't unpublish a theme that isn't yours");
            } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
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
pub fn install_warning(esp: bool) {
    println!(
            "Warning: When you install themes from the online repo, there is some danger. Please evaluate the theme files before loading the theme, and if you find any malicious theme, please report it on the theme's page at {} and it will be removed.",
            get_host()
        );
    if esp {
        println!(
                "This theme should be scrutinized more carefully as it includes a bash script which will be run automatically."
            );
    }
    println!("Thank you for helping keep the repo clean!");
}
pub fn check_tmp() -> bool {
    fs::metadata("/tmp").is_ok()
}
pub fn download_theme<N>(name: N, force: bool)
where
    N: Into<String>,
{
    let name = name.into();
    let mut tname = String::new();
    if check_tmp() {
        tname = tname + "/tmp/";
    }
    tname = tname + &name + ".tar";
    println!("{}", tname);
    let client = reqwest::Client::new();
    let res = client.get(&(get_host() + "/themes/repo/" + &name)).send();
    if res.is_ok() {
        let mut res = res.unwrap();
        if res.status().is_success() {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&tname)
                .expect("Couldn't write theme file");
            res.copy_to(&mut file).expect("Couldn't pipe to archive");
            println!("Downloaded theme.");
            if res.status() == reqwest::StatusCode::ALREADY_REPORTED && !force {
                print!(
                        "This theme has recently been reported, and has not been approved by an admin. It is not advisable to install this theme. Are you sure you would like to continue? (y/n)"
                    );
                io::stdout().flush().unwrap();
                let mut r = String::new();
                io::stdin().read_line(&mut r).unwrap();
                if r.trim() == "y" {
                    println!(
                            "Continuing. Please look carefully at the theme files in ~/.config/raven/themes/{} before loading this theme.",
                            name
                        );
                    import(tname.as_str());
                    println!("Imported theme. Removing archive.");
                    fs::remove_file(&tname).unwrap();
                    println!("Downloading metadata.");
                    let meta = get_metadata(name.as_str()).unwrap();
                    let mut st = load_store(name.as_str());
                    st.screenshot = meta.screen;
                    st.description = meta.description;
                    up_theme(st);
                    if fs::metadata(get_home() + "/.config/raven/themes/" + &name + "/script")
                        .is_ok()
                        || fs::metadata(get_home() + "/.config/raven/themes/" + &name + "/lemonbar")
                            .is_ok()
                    {
                        if !force {
                            install_warning(true);
                        }
                    } else {
                        if !force {
                            install_warning(false);
                        }
                    }
                } else {
                    println!("Removing downloaded archive.");
                    fs::remove_file(&tname).unwrap();
                }
            } else {
                if res.status() == reqwest::StatusCode::ALREADY_REPORTED {
                    print!(
                            "This theme has recently been reported, and has not been approved by an admin. It is not advisable to install this theme. Continuing because of --force."
                        );
                }
                import(tname.as_str());
                println!("Imported theme. Removing archive.");
                fs::remove_file(tname).unwrap();
                println!("Downloading metadata.");
                let meta = get_metadata(name.as_str()).unwrap();
                let mut st = load_store(name.as_str());
                st.screenshot = meta.screen;
                st.description = meta.description;
                up_theme(st);
                if fs::metadata(get_home() + "/.config/raven/themes/" + &name + "/script").is_ok()
                    || fs::metadata(get_home() + "/.config/raven/themes/" + &name + "/lemonbar")
                        .is_ok()
                {
                    if !force {
                        install_warning(true);
                    }
                } else {
                    if !force {
                        install_warning(false);
                    }
                }
            }
        } else {
            if res.status() == reqwest::StatusCode::NOT_FOUND {
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
pub fn login_user<N>(name: N, pass: N)
where
    N: Into<String>,
{
    let client = reqwest::Client::new();
    let res = client
        .get(&(get_host() + "/themes/user/login?name=" + &name.into() + "&pass=" + &pass.into()))
        .send();
    if res.is_ok() {
        let mut res = res.unwrap();
        if res.status().is_success() {
            println!("Successfully signed in. Writing login info to disk.");
            let info = res.json().unwrap();
            up_info(info);
        } else {
            if res.status() == reqwest::StatusCode::FORBIDDEN {
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
