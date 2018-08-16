mod ravenlib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gtk;
extern crate gio;
use ravenlib::rlib;
use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use gtk::{GtkWindowExt, WidgetExt};
use gio::ApplicationExt;
use gio::ApplicationExtManual;
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}
fn build_ui (application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_default_size(400,400);
    window.set_border_width(10);
    window.set_title("Raven GTK");
    window.set_position(gtk::WindowPosition::Center);
    let refresh = gtk::Button::new_with_label("Refresh Theme");
    let load = gtk::Button::new_with_label("Load Selected Theme");
    let new = gtk::Button::new_with_label("Create new theme");
    let rm = gtk::Button::new_with_label("Delete Theme");
    window.add(&refresh);
    window.show_all();
}
fn main () {
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty()).expect("Application init failed");
    application.connect_startup(|app| {
        println!("Building UI");
        build_ui(app);
    });
    application.connect_activate(|app| {
        println!("active");
    });
    application.run(&args().collect::<Vec<_>>());
}
