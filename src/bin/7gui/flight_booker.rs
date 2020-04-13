use glib::clone;
use gtk::{prelude::*, ButtonsType, MessageDialogBuilder};

struct BookingState {
    flight_valid: bool,
    return_valid: bool,
    start: chrono::NaiveDate,
    end: chrono::NaiveDate,
}

impl BookingState {
    fn disable(&self) -> bool {
        if self.flight_valid && self.return_valid {
            return false;
        }
        return true;
    }

    fn valid_dates(&self) -> bool {
        if self.start > self.end {
            return false;
        }
        true
    }
}

enum Message {
    ChangedType(bool),
    ValidationFlight(bool),
    ValidationReturn(bool),
    DateFlight(chrono::NaiveDate),
    DateReturn(chrono::NaiveDate),
    Book,
}

fn main() -> Result<(), glib::error::BoolError> {
    gtk::init()?;

    let today = chrono::Local::today();
    let mut booking_state = BookingState {
        flight_valid: true,
        return_valid: true,
        start: today.naive_utc(),
        end: today.naive_utc(),
    };

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let counter_ui = std::include_str!("flight_booker.ui");
    let builder = gtk::Builder::new_from_string(counter_ui);
    let window: gtk::Window = builder.get_object("win").unwrap();
    window.connect_destroy(|_win| {
        gtk::main_quit();
    });
    window.show_all();

    let flight_type: gtk::ComboBoxText =
        builder.get_object("flight_type").unwrap();

    let s2 = sender.clone();
    flight_type.connect_changed(move |combobox| {
        let selection = combobox.get_active_text().unwrap();
        if selection == "one-way flight" {
            s2.send(Message::ChangedType(false))
                .expect("Cannot send Message");
        } else {
            s2.send(Message::ChangedType(true))
                .expect("Cannot send Message");
        }
    });

    let book_btn: gtk::Button = builder.get_object("book_btn").unwrap();
    book_btn.connect_clicked(clone!(@strong sender => move |_btn| {
        sender.send(Message::Book).expect("Cannot send Message");
    }));

    let return_entry: gtk::Entry = builder.get_object("return_entry").unwrap();
    let flight_entry: gtk::Entry = builder.get_object("flight_entry").unwrap();
    let today_str = today.format("%d.%m.%Y").to_string();
    flight_entry.set_text(&today_str);
    return_entry.set_text(&today_str);

    let s1 = sender.clone();
    flight_entry.connect_changed(move |entry| {
        let text = entry.get_text().unwrap();
        match chrono::NaiveDate::parse_from_str(&text, "%d.%m.%Y") {
            Ok(date) => {
                entry.get_style_context().remove_class("error");
                s1.send(Message::ValidationFlight(true))
                    .expect("Cannot send Message");
                s1.send(Message::DateFlight(date))
                    .expect("Cannot send Message");
            }
            Err(_) => {
                entry.get_style_context().add_class("error");
                s1.send(Message::ValidationFlight(false))
                    .expect("Cannot send Message");
            }
        }
    });

    let s3 = sender.clone();
    return_entry.connect_changed(move |entry| {
        let text = entry.get_text().unwrap();
        match chrono::NaiveDate::parse_from_str(&text, "%d.%m.%Y") {
            Ok(date) => {
                entry.get_style_context().remove_class("error");
                s3.send(Message::ValidationReturn(true))
                    .expect("Cannot send Message");
                s3.send(Message::DateReturn(date))
                    .expect("Cannot send Message");
            }
            Err(_) => {
                entry.get_style_context().add_class("error");
                s3.send(Message::ValidationReturn(false))
                    .expect("Cannot send Message");
            }
        }
    });

    receiver.attach(None, move |msg| {
        match msg {
            Message::ChangedType(with_return) => {
                if with_return {
                    return_entry.set_sensitive(true);
                } else {
                    return_entry.set_sensitive(false);
                }
            }
            Message::ValidationFlight(valid) => {
                booking_state.flight_valid = valid
            }
            Message::ValidationReturn(valid) => {
                booking_state.return_valid = valid
            }
            Message::DateFlight(date) => booking_state.start = date,
            Message::DateReturn(date) => booking_state.end = date,
            Message::Book => {
                let dialog = MessageDialogBuilder::new()
                    .text("Hello World")
                    .buttons(ButtonsType::Close)
                    .build();
                dialog.connect_response(|dialog, _| {
                    dialog.destroy();
                });
                dialog.run();
            }
        }

        if booking_state.disable() || !booking_state.valid_dates() {
            book_btn.set_sensitive(false);
        } else {
            book_btn.set_sensitive(true);
        }

        glib::Continue(true)
    });

    gtk::main();
    Ok(())
}
