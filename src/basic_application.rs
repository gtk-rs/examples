//! # Basic Application Sample
//!
//! This sample demonstrates how to create a GTK application with a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gio;
extern crate gtk;

use gtk::prelude::*;

const APP_ID: &'static str = "org.gtk-rs.basic_app";
const APP_FLAGS: gio::ApplicationFlags = gio::APPLICATION_FLAGS_NONE;

#[cfg(feature = "gtk_3_10")]
fn new_app() -> Result<gtk::Application, ()> {
    gtk::Application::new(Some(APP_ID), APP_FLAGS)
}

#[cfg(not(feature = "gtk_3_10"))]
fn new_app() -> Result<gtk::Application, ()> {
    gtk::Application::new(APP_ID, APP_FLAGS)
}

fn main() {
    let app = new_app().unwrap();

    app.connect_activate(|app| {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        app.add_window(&window);

        window.set_title("Sample GTK+ Application");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        let button = gtk::Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("clicked");
        });
        window.add(&button);

        window.show_all();
    });

    // use -h to see usage options
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|x| &x[..]).collect();

    app.run(args.len() as i32, args.as_slice());
}
