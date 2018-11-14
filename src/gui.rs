extern crate azul;
extern crate ravenlib;
use azul::{prelude::*, widgets::{button::Button}};
use NodeType::*;
use ravenlib::*;
use themes::*;
use config::*;
struct DataModel {
    config: Config,
    themes: Vec<Theme>
}

impl Layout for DataModel {
    fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
        let mut themes_list = Dom::new(Div).with_id("themes-list");
        for theme in &self.themes {
            themes_list = themes_list.with_child(Dom::new(Label(format!("{}", theme.name))).with_class("theme-item"));
        }
        let theme_name = Dom::new(Label(format!("thjeme")));
        let new_but = Dom::new(Label(format!("new")));
        let mut cur_theme = Dom::new(Div).with_id("cur-theme").with_child(theme_name);
        let mut bottom_bar = Dom::new(Div).with_id("bottom-bar").with_child(new_but);
        let right = Dom::new(Div).with_id("right").with_child(cur_theme).with_child(bottom_bar);
        Dom::new(Div).with_id("main").with_child(themes_list).with_child(right)
    }
}

fn main() {
     macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/gui.css")) }
    println!("Starting GUI");
    let app = App::new(DataModel {
        config: get_config(),
        themes: load_themes()
    }, AppConfig::default());
    let css = Css::override_native(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
}
