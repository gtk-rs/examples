//! # ListBox and ListModel Sample
//!
//! This sample demonstrates how to use drag and drop to reorder Rows in gtk::ListBox
//! which is bind to a gio::ListStore model with a custom row type.
//!
//! It sets up a gtk::ListBox containing, per row, a label, spinbutton and
//! an edit button. The edit button allows to edit the underlying data structure
//! and changes are taking place immediately in the listbox by making use of GObject
//! property bindings.
//!
//! In addition it is possible to add new rows and delete old ones.

#[macro_use]
extern crate glib;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::ResponseType;

use std::cell::Cell;
use std::rc::{Rc, Weak};

use std::env::args;

use row_data::RowData;

// make moving clones into closures more convenient
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

// upgrade weak reference or return
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn build_ui(application: &gtk::Application) {
    // Add custom css style
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(CSS.as_bytes()).unwrap();
    gtk::StyleContext::add_provider_for_screen(
        gdk::Screen::get_default().as_ref().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = gtk::ApplicationWindow::new(application);

    window.set_title("ListBox Model Sample");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(320, 480);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create our list store and specify that the type stored in the
    // list should be the RowData GObject we define at the bottom
    let model = gio::ListStore::new(RowData::static_type());

    // And then create the UI part, the listbox and bind the list store
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.
    let listbox = gtk::ListBox::new();
    listbox.set_valign(gtk::Align::Start);
    // Create target for drag and drop if the listbox is a drag_area
    let targets = vec![gtk::TargetEntry::new(
        "CHANGE_POSITION",
        // This seams to have no effect
        gtk::TargetFlags::SAME_APP,
        0,
    )];
    // Create a communication channel between the dnd source and destination
    // We could also use the classic way of moving data between source and destiation, but this way
    // we can use all the advanges of rust and don't have to make sure that the data is correct.
    let channel = DndChannel::new(Cell::new(None));
    let channel_weak = Rc::downgrade(&channel);

    let model_weak = model.downgrade();
    listbox.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::MOVE);
    listbox.connect_drag_data_received(clone!(model_weak =>
                                           move |widget, _context, _x, y, _s, _info, _time| {
                                               let model = upgrade_weak!(model_weak);
                                               if let Some(data) = channel.take() {
                                                   if let Some(source) = model.get_object(data.position) {
                                                       if let Some(ref list) = widget.downcast_ref::<gtk::ListBox>() {
                                                           if let Some(row) = list.get_row_at_y(y) {
                                                           model.remove(data.position);
                                                           let index = row.get_index() as u32;
                                                           let alloc = row.get_allocation();

                                                           let index = if y < alloc.y + alloc.height/2 {
                                                               index
                                                           } else {
                                                               index + 1
                                                           };
                                                           model.insert(index, &source);
                                                           }
                                                       }
                                                   }
                                               }
                                           }
    ));

    listbox.connect_drag_leave(move |widget, _, _| {
        if let Some(ref list) = widget.downcast_ref::<gtk::ListBox>() {
            for row in list.get_children() {
                let style = row.get_style_context();
                style.remove_class("lower-mark");
                style.remove_class("upper-mark");
                style.remove_class("lower-mark-list-end");
                style.remove_class("upper-mark-list-start");
            }
        }
    });

    // We need to store the other row we highlight next to the hovered row
    let row_before: Rc<Cell<Option<glib::WeakRef<gtk::ListBoxRow>>>> = Rc::new(Cell::new(None));
    let row_after: Rc<Cell<Option<glib::WeakRef<gtk::ListBoxRow>>>> = Rc::new(Cell::new(None));
    listbox.connect_drag_motion(move |widget, _, _, y, _| {
        if let Some(row) = row_before.take().and_then(|w| Some(upgrade_weak!(w, None))) {
            let style = row.get_style_context();
            style.remove_class("lower-mark");
            style.remove_class("lower-mark-list-end");
        }
        if let Some(row) = row_after.take().and_then(|w| Some(upgrade_weak!(w, None))) {
            let style = row.get_style_context();
            style.remove_class("upper-mark");
            style.remove_class("upper-mark-list-start");
        }

        if let Some(ref list) = widget.downcast_ref::<gtk::ListBox>() {
            if let Some(row) = list.get_row_at_y(y) {
                let alloc = row.get_allocation();
                let style = row.get_style_context();

                if y < alloc.y + alloc.height / 2 {
                    style.add_class("upper-mark");
                    row_after.set(Some(row.downgrade()));
                    if let Some(prev) = get_previous_row(&list, &row) {
                        let style = prev.get_style_context();
                        style.add_class("lower-mark");
                        row_before.set(Some(prev.downgrade()));
                    } else {
                        style.add_class("upper-mark-list-start");
                    }
                } else {
                    style.add_class("lower-mark");
                    row_before.set(Some(row.downgrade()));

                    if let Some(next) = get_next_row(&list, &row) {
                        let style = next.get_style_context();
                        style.add_class("upper-mark");
                        row_after.set(Some(next.downgrade()));
                    } else {
                        style.add_class("lower-mark-list-end");
                    }
                };
            }
        }

        gtk::Inhibit(false)
    });

    let window_weak = window.downgrade();
    listbox.bind_model(Some(&model), clone!(window_weak => move |item| {
        let box_ = gtk::ListBoxRow::new();
        let revealer = gtk::Revealer::new();
        revealer.set_transition_duration(100);
        let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");

        let dnd_area = gtk::EventBox::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        connect_dnd_row(&dnd_area, &targets, channel_weak.clone());

        // Create the label and spin button that shows the two values
        // of the item. We bind the properties for the two values to the
        // corresponding properties of the widgets so that they are automatically
        // updated whenever the item is changing. By specifying SYNC_CREATE the
        // widget will automatically get the initial value of the item set.
        //
        // In case of the spin button the binding is bidirectional, that is any
        // change of value in the spin button will be automatically reflected in
        // the item.
        let label = gtk::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.pack_start(&label, true, true, 0);

        let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
        item.bind_property("count", &spin_button, "value")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
        hbox.pack_start(&spin_button, false, false, 0);

        // When the edit button is clicked, a new modal dialog is created for editing
        // the corresponding row
        let edit_button = gtk::Button::new_with_label("Edit");
        edit_button.connect_clicked(clone!(window_weak, item => move |_| {
            let window = upgrade_weak!(window_weak);

            let dialog = gtk::Dialog::new_with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
            &[("Close", ResponseType::Close)]);
            dialog.set_default_response(ResponseType::Close);
            dialog.connect_response(|dialog, _| dialog.destroy());

            let content_area = dialog.get_content_area();

            // Similarly to the label and spin button inside the listbox, the text entry
            // and spin button in the edit dialog are connected via property bindings to
            // the item. Any changes will be immediately reflected inside the item and
            // by the listbox
            let entry = gtk::Entry::new();
            item.bind_property("name", &entry, "text")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();

            // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
            let dialog_weak = dialog.downgrade();
            entry.connect_activate(move |_| {
                let dialog = upgrade_weak!(dialog_weak);
                dialog.response(ResponseType::Close);
            });
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
            item.bind_property("count", &spin_button, "value")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();
            content_area.add(&spin_button);

            dialog.show_all();
        }));
        hbox.pack_start(&edit_button, false, false, 0);

        dnd_area.add(&hbox);
        revealer.add(&dnd_area);
        box_.add(&revealer);

        // When a row is activated (select + enter) we simply emit the clicked
        // signal on the corresponding edit button to open the edit dialog
        let edit_button_weak = edit_button.downgrade();
        box_.connect_activate(move |_| {
            let edit_button = upgrade_weak!(edit_button_weak);
            edit_button.emit_clicked();
        });

        box_.show_all();

        //We need to make sure that the widget is in the listbox
        gtk::timeout_add(10, move || {
            revealer.set_reveal_child(true);
            glib::Continue(false)
        });

        box_.upcast::<gtk::Widget>()
    }));

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_window.add(&listbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // The add button opens a new dialog which is basically the same as the edit
    // dialog, except that we don't have a corresponding item yet at that point
    // and only create it once the Ok button in the dialog is clicked, and only
    // then add it to the model. Once added to the model, it will immediately
    // appear in the listbox UI
    let add_button = gtk::Button::new_with_label("Add");
    add_button.connect_clicked(clone!(window_weak, model => move |_| {
            let window = upgrade_weak!(window_weak);

            let dialog = gtk::Dialog::new_with_buttons(Some("Add Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
            dialog.set_default_response(ResponseType::Ok);

            let content_area = dialog.get_content_area();

            let entry = gtk::Entry::new();
            let dialog_weak = dialog.downgrade();
            entry.connect_activate(move |_| {
                let dialog = upgrade_weak!(dialog_weak);
                dialog.response(ResponseType::Ok);
            });
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
            content_area.add(&spin_button);

            dialog.connect_response(clone!(model, entry, spin_button => move |dialog, resp| {
                if let Some(text) = entry.get_text() {
                    if !text.is_empty() && resp == ResponseType::Ok {
                        model.append(&RowData::new(&text, spin_button.get_value() as u32));
                    }
                }
                dialog.destroy();
            }));

            dialog.show_all();
    }));

    hbox.add(&add_button);

    // Via the delete button we delete the item from the model that
    // is at the index of the selected row. Also deleting from the
    // model is immediately reflected in the listbox.
    let delete_button = gtk::Button::new_with_label("Delete");
    delete_button.connect_clicked(clone!(model, listbox => move |_| {
        let selected = listbox.get_selected_row();

        if let Some(selected) = selected {
            let idx = selected.get_index();
            model.remove(idx as u32);
        }
    }));
    hbox.add(&delete_button);

    vbox.pack_start(&hbox, false, false, 0);
    vbox.pack_start(&scrolled_window, true, true, 0);

    window.add(&vbox);

    for i in 0..10 {
        model.append(&RowData::new(&format!("Name {}", i), i * 10));
    }

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.listbox-model"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 2] = [
            subclass::Property("name", |name| {
                glib::ParamSpec::string(
                    name,
                    "Name",
                    "Name",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("count", |name| {
                glib::ParamSpec::uint(
                    name,
                    "Count",
                    "Count",
                    0,
                    100,
                    0, // Allowed range and default value
                    glib::ParamFlags::READWRITE,
                )
            }),
        ];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => {
                        let name = value.get();
                        self.name.replace(name);
                    }
                    subclass::Property("count", ..) => {
                        let count = value.get().expect("Got value of wrong type");
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                    subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, count: u32) -> RowData {
            glib::Object::new(Self::static_type(), &[("name", &name), ("count", &count)])
                .expect("Failed to create row data")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }
}

// return the previous visible row
fn get_previous_row(list: &gtk::ListBox, row: &gtk::ListBoxRow) -> Option<gtk::ListBoxRow> {
    if let Some(w) = list.get_row_at_index(row.get_index() - 1) {
        if w.get_child()
            .and_then(|w| w.downcast::<gtk::Revealer>().ok())
            .map_or(false, |r| !r.get_reveal_child())
        {
            get_previous_row(list, &w)
        } else {
            Some(w)
        }
    } else {
        None
    }
}

// return the next visible row
fn get_next_row(list: &gtk::ListBox, row: &gtk::ListBoxRow) -> Option<gtk::ListBoxRow> {
    if let Some(w) = list.get_row_at_index(row.get_index() + 1) {
        if w.get_child()
            .and_then(|w| w.downcast::<gtk::Revealer>().ok())
            .map_or(false, |r| !r.get_reveal_child())
        {
            get_next_row(list, &w)
        } else {
            Some(w)
        }
    } else {
        None
    }
}

fn connect_dnd_row(
    row: &gtk::EventBox,
    targets: &Vec<gtk::TargetEntry>,
    channel_weak: DndChannelWeak,
) {
    row.drag_source_set(
        gdk::ModifierType::BUTTON1_MASK,
        targets,
        gdk::DragAction::MOVE,
    );

    // Event fired when the user starts dragging, we set the ListBoxRow as the dragging icon
    row.connect_drag_begin(move |w, ctx| {
        if let Some(w) = w
            .get_parent()
            .and_then(|w| w.downcast::<gtk::Revealer>().ok())
        {
            let ww = w.get_allocated_width();
            let wh = w.get_allocated_height();
            let image = cairo::ImageSurface::create(cairo::Format::ARgb32, ww, wh).unwrap();
            let g = cairo::Context::new(&image);
            //TODO use theme color as the background or add a temporarily style class
            g.set_source_rgba(1.0, 1.0, 1.0, 0.8);
            g.rectangle(0.0, 0.0, ww as f64, wh as f64);
            g.fill();

            w.draw(&g);

            //TODO fix positioning
            //https://stackoverflow.com/questions/24844489/how-to-use-gdk-device-get-position
            ctx.drag_set_icon_surface(&image);
            w.set_reveal_child(false);
            if let Some(w) = w.get_parent() {
                let style = w.get_style_context();
                style.add_class("dragging");
            }
        }
    });

    row.connect_drag_end(move |w, _context| {
        // Show the row again if it wasn't removed
        if let Some(w) = w
            .get_parent()
            .and_then(|w| w.downcast::<gtk::Revealer>().ok())
        {
            w.set_reveal_child(true);
            if let Some(w) = w.get_parent() {
                let style = w.get_style_context();
                style.remove_class("dragging");
            }
        }
    });

    row.connect_drag_data_get(move |w, _context, data, _info, _time| {
        let atom = gdk::Atom::intern("CHANGE_POSITION");
        if data.get_target() == atom {
            if let Some(index) = w
                .get_parent()
                .and_then(|w| w.get_parent())
                .and_then(|w| w.downcast::<gtk::ListBoxRow>().ok())
                .and_then(|w| Some(w.get_index() as u32))
            {
                let channel = upgrade_weak!(channel_weak);
                channel.set(Some(DndChannelData::new(index)));
                // We need to set data so drag_data_received is fired
                data.set(&atom, 0, &[]);
            }
        }
    });
}

pub type DndChannel = Rc<Cell<Option<DndChannelData>>>;
pub type DndChannelWeak = Weak<Cell<Option<DndChannelData>>>;

// This is the data we pass to the dnd destiation, since we are using a rust struct we could and
// also much more information
pub struct DndChannelData {
    position: u32,
}

impl DndChannelData {
    pub fn new(position: u32) -> Self {
        DndChannelData { position }
    }
}

static CSS: &'static str = "
    list {
        padding: 2px 0;
        box-shadow: none;
        border: 1px solid #cdc7c2;
    }
    list row {
        padding: 0px;
    }
    .upper-mark {
        /* top */
        box-shadow: 0 -1px 0 #4e9a06;
    }
    .lower-mark {
        /* bottom */
        box-shadow: 0 1px 0 #4e9a06;
    }
    .lower-mark-list-end {
        box-shadow: 0 2px 0 #4e9a06;
    }
    .upper-mark-list-start {
        box-shadow: 0 -2px 0 #4e9a06;
    }
    row > revealer {
	padding: 10px;
    }

    row:not(:last-child):not(.dragging){
	border-bottom: 1px solid #cdc7c2;
    }
";
