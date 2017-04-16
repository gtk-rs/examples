//! # RadioButton Sample
//!
//! This sample demonstrates how to create RadioButton's and how there first parameter work

#![crate_type = "bin"]

extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Radio Button example");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    // RadioButton "stand alone"
    // Each Radio Button is independent of each other.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let radio_size_10 = gtk::RadioButton::new_with_label_from_widget(None, "10x10");
    let radio_size_20 = gtk::RadioButton::new_with_label_from_widget(None, "20x20");
    let radio_size_30 = gtk::RadioButton::new_with_label_from_widget(None, "30x30");
    let radio_size_40 = gtk::RadioButton::new_with_label_from_widget(None, "40x40");

    vbox.pack_start(&gtk::Label::new("RadioButton stand alone"), true, false, 0);
    vbox.pack_start(&radio_size_10, false, false, 0);
    vbox.pack_start(&radio_size_20, false, false, 0);
    vbox.pack_start(&radio_size_30, false, false, 0);
    vbox.pack_start(&radio_size_40, false, false, 0);

    hbox.pack_start(&vbox, false, false, 0);

    // RadioButton child of `10x10`
    // Here all Radio Button are connected to each other. Only one can be set at once.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let radio_size_10 = gtk::RadioButton::new_with_label_from_widget(None, "10x10");
    let radio_size_20 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "20x20");
    let radio_size_30 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "30x30");
    let radio_size_40 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "40x40");

    vbox.pack_start(&gtk::Label::new("RadioButton child of `10x10`"), true, false, 0);
    vbox.pack_start(&radio_size_10, false, false, 0);
    vbox.pack_start(&radio_size_20, false, false, 0);
    vbox.pack_start(&radio_size_30, false, false, 0);
    vbox.pack_start(&radio_size_40, false, false, 0);

    hbox.pack_end(&vbox, false, false, 0);
    window.add(&hbox);

    window.show_all();
    gtk::main();
}
