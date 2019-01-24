extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gdk::{Atom, DragAction, DragContext, ModifierType, Screen};
use gio::prelude::*;
use gtk::*;
use std::env::args;

fn main() {
    let app = gtk::Application::new("com.github.gtk-rs.examples.basic", Default::default())
        .expect("Initialization failed...");

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run(&args().collect::<Vec<_>>());
}

fn build_ui(app: &gtk::Application) {
    let css_provider = CssProvider::new();
    css_provider.load_from_data(CSS.as_bytes()).unwrap();
    StyleContext::add_provider_for_screen(
        Screen::get_default().as_ref().unwrap(),
        &css_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = gtk::ApplicationWindow::new(app);
    window.set_title("A drag-and-drop example");
    window.set_default_size(350, 500);

    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::None);

    window.add(&list_box);

    // Add rows
    for i in 0..20 {
        list_box.add(&create_row(i));
    }

    window.show_all();
}

fn create_row(i: i32) -> ListBoxRow {
    let list_box_row = ListBoxRow::new();
    let box_ = Box::new(Orientation::Horizontal, 5);
    box_.set_margin_start(10);
    box_.set_margin_end(10);
    list_box_row.add(&box_);

    let event_box = EventBox::new();
    let image = Image::new_from_icon_name("open-menu-symbolic", IconSize::Button);
    event_box.add(&image);
    box_.add(&event_box);

    let label_text = format!("Row {}", i);
    let label = Label::new(Some(label_text.as_str()));

    box_.add(&label);

    // Drag stuff
    //
    // Make the event box draggable. Triggered with left mouse button.
    //
    let drag_targets: [TargetEntry; 1] = [TargetEntry::new(
        "GTK_LIST_BOX_ROW", // I don't know how this string is used
        TargetFlags::SAME_APP,
        0,
    )];
    event_box.drag_source_set(ModifierType::BUTTON1_MASK, &drag_targets, DragAction::MOVE);
    // drag_begin will be called when we start dragging. drag_begin is where we call rendering the
    // widget being dragged next to mouse cursor.
    event_box.connect_drag_begin(drag_begin);
    // We don't use this, but just to demonstrate that things can be done when the drag ends.
    event_box.connect_drag_end(drag_end);
    // See https://developer.gnome.org/gtkmm-tutorial/stable/sec-dnd-signals.html.en
    // Called when a source is being dragged and the target calls get_data()
    event_box.connect_drag_data_get(drag_data_get);
    // Set the same box as a drag destination.
    event_box.drag_dest_set(DestDefaults::ALL, &drag_targets, DragAction::MOVE);
    // Called when get_data() returns
    event_box.connect_drag_data_received(drag_data_received);

    list_box_row
}

fn drag_begin<W: IsA<Widget>>(w: &W, ctx: &DragContext) {
    println!("drag_begin({:?})", w);

    // Get containing ListBoxRow and render it next to the cursor while being dragged
    match w.get_ancestor(ListBoxRow::static_type()) {
        None => {
            println!("Can't get ancestor");
        }
        Some(ancestor) => {
            let alloc = ancestor.get_allocation();
            let surface: cairo::ImageSurface =
                cairo::ImageSurface::create(cairo::Format::ARgb32, alloc.width, alloc.height)
                    .unwrap();
            let surface_: &cairo::Surface = surface.as_ref();

            {
                ancestor.get_style_context().add_class("drag-icon");
                let surface_ctx = cairo::Context::new(surface_);
                ancestor.draw(&surface_ctx);
                ancestor.get_style_context().remove_class("drag-icon");
            }

            let (x, y) = w.translate_coordinates(&ancestor, 0, 0).unwrap();
            surface.set_device_offset(-x as f64, -y as f64);
            ctx.drag_set_icon_surface(surface_);
        }
    }
}

fn drag_end<W: IsA<Widget>>(w: &W, _ctx: &DragContext) {
    println!("drag_end({:?})", w);
}

fn drag_data_get<W: IsA<Widget>>(w: &W, _ctx: &DragContext, sel: &SelectionData, _: u32, _: u32) {
    println!("drag_data_get({:?}", w);

    // Get containing ListBoxRow index and put that into SelectionData, to be read in
    // drag_data_received below
    let list_box_row = w.get_ancestor(ListBoxRow::static_type()).unwrap();
    // Downcast it
    let list_box_row = list_box_row.downcast_ref::<ListBoxRow>().unwrap();

    let row_idx = list_box_row.get_index();
    sel.set(&Atom::intern(""), 32, &row_idx.to_ne_bytes());
}

fn drag_data_received<W: IsA<Widget>>(
    target: &W,
    _ctx: &DragContext,
    _x: i32,
    _y: i32,
    sel: &SelectionData,
    _info: u32,
    _time: u32,
) {
    println!("drag_data_received({:?})", target);

    // Index of the widget being dragged, as we stored in the SelectionData in drag_data_get()
    // above.
    let row_idx: i32 = {
        let bytes = sel.get_data();
        assert!(bytes.len() == 4);
        let bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        i32::from_ne_bytes(bytes)
    };

    // Get the list box that contains both the source and the target
    let list_box = target.get_ancestor(ListBox::static_type()).unwrap();
    // Downcast it
    let list_box: &ListBox = list_box.downcast_ref::<ListBox>().unwrap();

    // Get the source widget from the list box
    let source = list_box.get_row_at_index(row_idx).unwrap();
    // Downcast it
    let source = source.downcast_ref::<ListBoxRow>().unwrap();

    // The target is a EventBox, get its containing ListBoxRow
    let target = target.get_ancestor(ListBoxRow::static_type()).unwrap();
    // Downcast it
    let target = target.downcast_ref::<ListBoxRow>().unwrap();

    if source == target {
        println!("Source and target are the same. Aborting drag.");
        return;
    }

    // Remove source from the list box
    list_box.remove(source);
    // Add it next to target
    let target_row = target.get_index();
    list_box.insert(source, target_row);
}

static CSS: &'static str = "
    .drag-icon {
        background: white;
        border: 1px solid black;
    }
";
