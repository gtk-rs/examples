//! # Textview and TextTag Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` and a `textview` to this `window` and how to change text style
//! on click

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    // Sample text
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor 
    incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation 
    ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit 
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat 
    cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Text View Apply Tag");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 300);

    // This button will change the style of selected text
    let button = gtk::Button::new_with_label("Style selected");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // Create a text tag that will change the appearance of text
    let text_tag = gtk::TextTag::new(Some("bg"));
    text_tag.set_property_background(Some("grey"));
    text_tag.set_property_foreground(Some("orange"));
    text_tag.set_property_style(pango::Style::Oblique);

    //We need to add our newly create tag to TextTagTable
    let text_tag_table = gtk::TextTagTable::new();
    text_tag_table.add(&text_tag);

    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);

    //Build a buffer that contains the tag table and text
    let buffer = gtk::TextBufferBuilder::new()
        .tag_table(&text_tag_table)
        .text(text)
        .build();

    // Set the text view buffer to our custom buffer
    text_view.set_buffer(Some(&buffer));

    let sw = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    sw.set_min_content_height(300);

    // Clone gtk objects to be used in closure
    let buffer_clone = buffer.clone();
    let tag_clone = text_tag.clone();
    button.connect_clicked(move |_| {
        // get selected text bounds
        let selection_bounds: Option<(gtk::TextIter, gtk::TextIter)> =
            buffer_clone.get_selection_bounds();
        // If text is selected, apply tag
        if let Some((start, end)) = selection_bounds {
            buffer_clone.apply_tag(&tag_clone, &start, &end);
        }
    });

    sw.add(&text_view);

    vbox.add(&button);
    vbox.add(&sw);
    window.add(&vbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.textviewtag"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
