//! # IconView Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `IconView` to this `window` and how to se `model` of the `IconView`

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::process;

enum IconViewColumnType {
    TextColumn = 0,
    PixbufColumn = 1
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let icons: [&'static str; 3] = ["edit-cut", "edit-paste", "edit-copy"];

    window.set_title("IconView Example");
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

    icon_view.set_model(Some(&icon_view_model));
    icon_view.set_text_column(IconViewColumnType::TextColumn as i32);
    icon_view.set_pixbuf_column(IconViewColumnType::PixbufColumn as i32);

    let icon = gtk::IconTheme::get_default();
    if let Some(i) = icon {
        for x in &icons {
            let result = i.load_icon(x, 64, gtk::IconLookupFlags::empty());
            match result {
                Ok(r) => {
                    /* arguments are position: 
                     Option<u32>,
                     columns: &[u32],
                     values: &[&dyn ToValue]
                     Columns are first defined in icon_view.set_text_column and icon_view.set_pixbuf_column
                    */
                    icon_view_model.insert_with_values(None, &[IconViewColumnType::TextColumn as u32, IconViewColumnType::PixbufColumn as u32], &[&String::from("Label"), &r]);
                },
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            }
        }

    }
    
    window.add(&icon_view);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.iconview_example"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
