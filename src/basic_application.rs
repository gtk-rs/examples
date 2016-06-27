//! # Basic Application Sample
//!
//! This sample demonstrates how to create a GTK application with a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gio;
extern crate gtk;

use gtk::Builder;
use gtk::prelude::*;

const APP_ID: &'static str = "org.gtk-rs.basic_app";
const APP_FLAGS: gio::ApplicationFlags = gio::APPLICATION_FLAGS_NONE;

#[cfg(feature = "gtk_3_6")]
fn new_app() -> Result<gtk::Application, ()> {
    gtk::Application::new(Some(APP_ID), APP_FLAGS)
}

#[cfg(not(feature = "gtk_3_6"))]
fn new_app() -> Result<gtk::Application, ()> {
    gtk::Application::new(APP_ID, APP_FLAGS)
}

fn main() {
    let app = new_app().unwrap();

    app.connect_activate(|app| {
        let glade_src = include_str!("basic.glade");
        let builder = Builder::new();
        builder.add_from_string(glade_src).unwrap();

        let window: gtk::Window = builder.get_object("window").unwrap();
        app.add_window(&window);

        let button: gtk::Button = builder.get_object("clickme_button").unwrap();
        button.connect_clicked(|_| {
            println!("clicked");
        });

        window.show_all();
    });

    // use -h to see usage options
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|x| &x[..]).collect();

    app.run(args.len() as i32, args.as_slice());
}
