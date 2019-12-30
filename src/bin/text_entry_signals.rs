//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `GtkEntry` to this `window` and how to connect signals with
//! actions for the text field using glib

extern crate gio;
extern crate gtk;
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

use glib::value::Value;
use glib::value::TypedValue;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Connect signals to text field");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let text_field = gtk::Entry::new();
    vbox.pack_start(&text_field, false, true, 0);

    // Using glib to connect
    // let _text_field_chaged_result = text_field.connect("changed", true, move |val| {
    //     let value: Value = val[0].clone();
    //     let type_value_entry: TypedValue<gtk::Entry> = value.downcast::<gtk::Entry>().unwrap();
    //     let entry_text = type_value_entry.get().unwrap().get_text();

    //     match entry_text {
    //         None => {},
    //         Some(s) => {
    //             println!("{}", &s);
    //         }
    //     }
    //     return None;
    // });

    let _text_field_chaged_result = text_field.connect_changed(move |val| {
        let entry_text = val.get_text();
        match entry_text {
            None => {},
            Some(s) => {
                println!("{}", &s);
            }
        }
    });

    // let button = gtk::Button::new_with_label("Click me!");

    window.add(&vbox);


    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.text_entry_signals"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
