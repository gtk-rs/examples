extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Configure button as drag source
    let button = gtk::Button::new_with_label("Drag here");
    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TargetFlags::empty(), 0),
        gtk::TargetEntry::new("text/plain", gtk::TargetFlags::empty(), 0),
    ];
    button.drag_source_set(gdk::MODIFIER_MASK,
                           &targets,
                           gdk::ACTION_COPY);
    button.connect_drag_data_get(|_, _, s, _, _| {
        println!("drag-data-get");
        let data = "I'm data!";
        s.set_text(data, data.len() as i32);
    });
    button.connect_drag_begin(|_, c| {
        println!("drag-begin");
        for t in c.list_targets() {
            println!("{}", t.name());
        }
    });
    button.connect_drag_end(|_, _| {
        println!("drag-end");
    });

    // Configure label as drag destination
    let label = gtk::Label::new("Drop here");
    label.drag_dest_set(gtk::DEST_DEFAULT_ALL,
                        &targets,
                        gdk::ACTION_COPY);
    label.connect_drag_data_received(|w, d, _, _, s, _, t| {
        println!("drag-data-received");
        if s.get_length() > 0 && s.get_format() == 8 {
            w.set_text(&s.get_text().unwrap());
        }
        d.drag_finish(false, false, t);
    });
    button.connect_drag_drop(|_, _, _, _, _| {
        println!("drag-drop");
        true
    });

    // Pack widgets into the window and display everything
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&button, true, true, 0);
    hbox.pack_start(&label, true, true, 0);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.add(&hbox);
    window.show_all();

    // GTK & main window boilerplate
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
