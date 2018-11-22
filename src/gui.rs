extern crate azul;
extern crate ravenlib;
extern crate reqwest;
use azul::window_state::WindowSize;
use azul::{prelude::*, widgets::button::Button};
use config::*;
use ravenlib::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use themes::*;
use NodeType::*;
struct DataModel {
    config: Config,
    themes: Vec<Theme>,
    selected_theme: Option<usize>,
    text: Vec<TextId>,
    screenshots: Vec<Option<String>>,
}
impl Layout for DataModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let mut set = vec![On::MouseUp];
        let buts = self
            .themes
            .iter()
            .enumerate()
            .map(|(i, theme)| NodeData {
                node_type: NodeType::Label(theme.name.clone()),
                classes: if self.selected_theme == Some(i) {
                    vec!["theme-item".into(), "selected".into()]
                } else {
                    vec!["theme-item".into()]
                },
                force_enable_hit_test: set.clone(),
                ..Default::default()
            })
            .collect::<Dom<Self>>()
            .with_id("themes-list")
            .with_callback(On::MouseUp, Callback(select_theme));
        let delete_button = Button::with_label("Delete Theme")
            .dom().with_class("bot-button")
            .with_callback(On::MouseUp, Callback(delete_callback));
        let load_button = Button::with_label("Load Theme")
            .dom().with_class("bot-button")
            .with_callback(On::MouseUp, Callback(load_callback));
        let refresh_button = Button::with_label("Refresh Last Theme")
            .dom().with_class("bot-button")
            .with_callback(On::MouseUp, Callback(refresh_callback));
        let mut cur_theme = Dom::new(Div).with_id("cur-theme");
        let online_button = Button::with_label("View on ThemeHub")
            .dom().with_class("bot-button")
            .with_callback(On::MouseUp, Callback(online_callback));
        let open_button = Button::with_label("View in File Manager").dom().with_class("bot-button").with_callback(On::MouseUp, Callback(open_callback));
        if self.selected_theme.is_some() {
            let theme = &self.themes[self.selected_theme.unwrap()];
            let name = Dom::new(Label(theme.name.clone())).with_class("theme-name");
            let option_list =
                Dom::new(Text(self.text[self.selected_theme.unwrap()])).with_class("option-list");
            cur_theme = cur_theme.with_child(name);
            if theme.screenshot != default_screen() && theme.screenshot.len() > 0 {
                if info.resources.has_image(theme.name.clone()) {
                    let screenshot =
                        Dom::new(Image(info.resources.get_image(theme.name.clone()).unwrap()))
                            .with_class("theme-image");
                    cur_theme = cur_theme.with_child(screenshot);
                }
            }
            cur_theme = cur_theme.with_child(option_list);
        } else {
            cur_theme = cur_theme.with_child(Dom::new(Label(format!("No Theme selected"))));
        }
        let mut bottom_bar = Dom::new(Div)
            .with_id("bottom-bar")
            .with_child(online_button)
            .with_child(open_button)
            .with_child(refresh_button)
            .with_child(load_button)
            .with_child(delete_button);
        let right = Dom::new(Div)
            .with_id("right")
            .with_child(cur_theme)
            .with_child(bottom_bar);
        Dom::new(Div)
            .with_id("main")
            .with_child(buts)
            .with_child(right)
    }
}
fn load_callback(state: &mut AppState<DataModel>, event: WindowEvent<DataModel>) -> UpdateScreen {
    let data = state.data.lock().unwrap();
    if data.selected_theme.is_some() {
        println!(
            "Loading theme {}",
            data.themes[data.selected_theme.unwrap()].name
        );
        run_theme(&data.themes[data.selected_theme.unwrap()]);
        UpdateScreen::Redraw
    } else {
        UpdateScreen::DontRedraw
    }
}
fn open_callback(state: &mut AppState<DataModel>, event: WindowEvent<DataModel>) -> UpdateScreen {
    let data = state.data.lock().unwrap();
    if data.selected_theme.is_some() {
        let path = get_home()+"/.config/raven/themes/"+&data.themes[data.selected_theme.unwrap()].name;
        println!("{}",path);
        let output = Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Couldn't use xdg-open to open folder");
    }
    UpdateScreen::DontRedraw

}
fn delete_callback(state: &mut AppState<DataModel>, event: WindowEvent<DataModel>) -> UpdateScreen {
    let mut up = UpdateScreen::DontRedraw;
    state.data.modify(|data| {
        if data.selected_theme.is_some() {
            println!(
                "Deleting theme {}",
                data.themes[data.selected_theme.unwrap()].name
            );
            del_theme(data.themes[data.selected_theme.unwrap()].name.as_str());
            data.themes.remove(data.selected_theme.unwrap());
            data.selected_theme = Some(0);
            up = UpdateScreen::Redraw
        }
    });
    up
}
fn refresh_callback(
    state: &mut AppState<DataModel>,
    event: WindowEvent<DataModel>,
) -> UpdateScreen {
    refresh_theme(state.data.lock().unwrap().config.last.clone());
    UpdateScreen::DontRedraw
}
fn online_callback(state: &mut AppState<DataModel>, event: WindowEvent<DataModel>) -> UpdateScreen {
    let data = state.data.lock().unwrap();
    if data.selected_theme.is_some() {
        let host = data.config.host.clone();
        let uri = host + "/themes/view/" + &data.themes[data.selected_theme.unwrap()].name;
        let output = Command::new("xdg-open")
            .arg(uri)
            .output()
            .expect("Couldn't use xdg-open to launch website");
    }
    UpdateScreen::DontRedraw
}
fn select_theme(
    app_state: &mut AppState<DataModel>,
    event: WindowEvent<DataModel>,
) -> UpdateScreen {
    println!("{}", event.hit_dom_node);
    let selected = event
        .get_first_hit_child(event.hit_dom_node, On::MouseUp)
        .and_then(|x| Some(x.0));
    let mut should_redraw = UpdateScreen::DontRedraw;
    app_state.data.modify(|state| {
        if selected.is_some() && selected != state.selected_theme {
            state.selected_theme = selected;
            should_redraw = UpdateScreen::Redraw;
        }
        println!("Selected theme: {:?}", state.selected_theme);
    });
    should_redraw
}
fn main() {
    if fs::metadata(get_home() + "/.config/raven/screenshots").is_err() {
        let cres = fs::create_dir(get_home() + "/.config/raven/screenshots");
        if cres.is_err() {
            println!(
                "Failed to init screenshot directory. Error Message: {:?}\n",
                cres
            );
        }
    }
    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/gui.css")
        };
    }
    println!("Starting GUI");
    let mut themes = load_themes();
    let mut app = App::new(
        DataModel {
            config: get_config(),
            selected_theme: None,
            themes: themes.clone(),
            text: vec![],
            screenshots: vec![],
        },
        AppConfig::default(),
    );

    let font_id = FontId::BuiltinFont("sans-serif".into());
    for (i, theme) in themes.iter().enumerate() {
        if theme.screenshot != default_screen() {
            let mut buf: Vec<u8> = vec![];
            let spath = get_home()
                + "/.config/raven/screenshots/"
                + &theme.screenshot.clone().replace("/", "").replace(":", "");
            if fs::metadata(&spath).is_err() {
                print!(
                    "Downloading {}'s screenshot from {}",
                    theme.name, theme.screenshot
                );
                let mut fd = fs::File::create(&spath).unwrap();
                let res = reqwest::get(&theme.screenshot.clone());
                if res.is_ok() {
                    let r = res.unwrap().read_to_end(&mut buf);
                    if r.is_err() {
                        println!("Failed reading. Error Message: \n{:?}", r);
                        continue;
                    } else {
                        fd.seek(SeekFrom::Start(0));
                        fd.write_all(&mut buf).expect("Couldn't write to file");
                    }
                } else {
                    println!("Failed downloading. Error Message: \n{:?}", res);
                    continue;
                }
            } else {
                let mut fd = fs::File::open(&spath).unwrap();
                fd.read_to_end(&mut buf).expect("Couldn't read file");
            }
            let ires = app.add_image(
                theme.name.clone(),
                &mut buf.as_slice(),
                ImageType::GuessImageFormat,
            );
            app.app_state.data.modify(|state| {
                state.screenshots.resize(i + 1, Some(String::new()));
                state.screenshots[i] = Some(theme.name.clone());
            });
        }
        let mut text = String::new();
        if theme.description != default_desc() && theme.description.len() > 0 {
            text = text + "Description:\n\n" + theme.description.as_str() + "\n\n";
        }
        text += "Options Added: \n\n";
        let option_string = theme
            .options
            .iter()
            .fold(text, |acc, opt| acc + &format!("- {}\n", opt));
        let text_id = app.add_text_cached(option_string, &font_id, PixelValue::px(10.0), None);
        app.app_state.data.modify(|state| {
            state.text.push(text_id);
        });
    }
    let mut create_options = WindowCreateOptions::default();
    let mut size = WindowSize::default();
    size.dimensions = LogicalSize::new(980.0, 600.0);
    create_options.state.title = String::from("graven");
    create_options.state.size = size;
    let css = Css::override_native(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(create_options, css).unwrap();
    app.run(window).unwrap();
}
