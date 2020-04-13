use gtk::prelude::*;
use gtk::Window;

enum Message {
    Increment,
}

fn main() -> Result<(), glib::error::BoolError> {
    gtk::init()?;

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let counter_ui = std::include_str!("counter.ui");
    let builder = gtk::Builder::new_from_string(counter_ui);

    let window: Window = builder.get_object("win").unwrap();
    window.connect_destroy(|_win| {
        gtk::main_quit();
    });
    window.show_all();

    let counter_lbl: gtk::Label = builder.get_object("lbl").unwrap();

    let counter_btn: gtk::Button = builder.get_object("btn").unwrap();
    counter_btn.connect_clicked(move |_btn| {
        sender
            .send(Message::Increment)
            .expect("Cannot send Message");
    });

    let mut i = 0;
    receiver.attach(None, move |msg| {
        match msg {
            Message::Increment => {
                i = i + 1;
                counter_lbl.set_text(&i.to_string());
            }
        }
        glib::Continue(true)
    });

    gtk::main();

    Ok(())
}
