//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;
extern crate gdk_pixbuf;

use gtk::{
    BoxExt, ContainerExt, Dialog, DialogExt, Inhibit, Label, Menu, MenuBar, MenuItem, MenuItemExt,
    MenuShellExt, WidgetExt, WidgetSignals, Window, WindowExt, WindowPosition, WindowType
};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("MenuBar example");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let h_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");

    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    let other_menu = Menu::new();
    let sub_other_menu = Menu::new();
    let other = MenuItem::new_with_label("Another");
    let sub_other = MenuItem::new_with_label("Sub another");
    let sub_other2 = MenuItem::new_with_label("Sub another 2");
    let sub_sub_other2 = MenuItem::new_with_label("Sub sub another 2");
    let sub_sub_other2_2 = MenuItem::new_with_label("Sub sub another2 2");

    sub_other_menu.append(&sub_sub_other2);
    sub_other_menu.append(&sub_sub_other2_2);
    sub_other2.set_submenu(Some(&sub_other_menu));
    other_menu.append(&sub_other);
    other_menu.append(&sub_other2);
    other.set_submenu(Some(&other_menu));
    menu_bar.append(&other);

    quit.connect_activate(|_| {
        gtk::main_quit();
    });

    let label = Label::new(Some("MenuBar example"));

    h_box.pack_start(&menu_bar, false, false, 0);
    h_box.pack_start(&label, true, true, 0);
    window.add(&h_box);
    window.show_all();

    about.connect_activate(move |_| {
        let p = Dialog::new_with_buttons(Some("About"),
                                         Some(&window),
                                         gtk::DIALOG_MODAL,
                                         &[("Ok", gtk::ResponseType::Ok as i32)]);
        let area = p.get_content_area();
        let label = Label::new(Some("MenuBar example"));
        area.add(&label);
        p.show_all();
        p.connect_response(|w, _| {
            w.destroy();
        });
    });
    gtk::main();
}
