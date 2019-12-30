//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Fucking IconView In Rust");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let icon_view = gtk::IconView::new();
    icon_view.set_item_padding(0);
    icon_view.set_columns(4);
    icon_view.set_column_spacing(0);
    icon_view.set_selection_mode(gtk::SelectionMode::Single);
    icon_view.set_activate_on_single_click(false);

    let col_types: [glib::Type; 2] = [glib::Type::String, gdk_pixbuf::Pixbuf::static_type()];
    let icon_view_model = gtk::ListStore::new(&col_types);

    // icon_view_model.append();
    icon_view.set_model(Some(&icon_view_model));
    icon_view.set_text_column(0);
    icon_view.set_pixbuf_column(1);

    let icon = gtk::IconTheme::get_default();
    if let Some(i) = icon {
        let result = i.load_icon("edit-cut", 64, gtk::IconLookupFlags::empty());
        match result {
            Ok(r) => {
                icon_view_model.insert_with_values(None, &[0, 1], &[&String::from("test"), &r]);
            },
            Err(e) => {}
        }
    }
    
    window.add(&icon_view);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
