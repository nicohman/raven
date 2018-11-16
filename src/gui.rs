extern crate azul;
extern crate ravenlib;
use azul::{prelude::*, widgets::button::Button};
use config::*;
use ravenlib::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;
use themes::*;
use NodeType::*;
use std::fs;
struct DataModel {
    config: Config,
    themes: Vec<Theme>,
    selected_theme: Option<usize>,
    text: Vec<TextId>
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
        let new_but = Dom::new(Label(format!("new")));
        let load_button = Button::with_label("Load Theme")
            .dom()
            .with_callback(On::MouseUp, Callback(load_callback));
        let mut cur_theme = Dom::new(Div).with_id("cur-theme");
        if self.selected_theme.is_some() {
            let theme = &self.themes[self.selected_theme.unwrap()];

            let name = Dom::new(Label(theme.name.clone())).with_class("theme-name");
            let option_list = Dom::new(Text(self.text[self.selected_theme.unwrap()])).with_class("option-list");
            let screenshot = Dom::new(Image(info.resources.get_image(theme.name.clone()).unwrap())).with_class("theme-image");
            cur_theme = cur_theme.with_child(name).with_child(screenshot).with_child(option_list);
        } else {
            cur_theme = cur_theme.with_child(Dom::new(Label(format!("No Theme selected"))));
        }
        let mut bottom_bar = Dom::new(Div)
            .with_id("bottom-bar")
            .with_child(new_but)
            .with_child(load_button);
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
            println!("Changed")
        }
        println!("selected item: {:?}", state.selected_theme);
    });
    should_redraw
}
fn main() {
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
            selected_theme: Some(0),
            themes: load_themes(),
            text:vec![]
        },
        AppConfig::default(),
    );
    let font_id = FontId::BuiltinFont("sans-serif".into());
    let test = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test.png");
    for theme in themes {
        let mut fd = fs::File::open(test).unwrap();
        app.add_image(theme.name.clone(), &mut fd, ImageType::Png);
        let option_string = theme.options.iter().fold("".to_string(), |acc, opt| {
            acc + &format!("- {}\n", opt)
        });
        let text_id = app.add_text_cached(option_string, &font_id,PixelValue::px(10.0), None);
        app.app_state.data.modify(|state| {
            state.text.push(text_id);
        });
    }
    let css = Css::override_native(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
}
