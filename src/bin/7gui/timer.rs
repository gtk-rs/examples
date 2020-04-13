use glib::clone;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use timer::Timer;

enum Message {
    CountFire(f64),
    MaxAdjusted(f64),
}

// counter from 0 .. 30
fn main() -> Result<(), glib::error::BoolError> {
    gtk::init()?;

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let timer_ui = std::include_str!("timer.ui");
    let builder = gtk::Builder::new_from_string(timer_ui);
    let window: gtk::Window = builder.get_object("win").unwrap();
    window.connect_destroy(|_win| {
        gtk::main_quit();
    });
    window.show_all();

    let count_lbl: gtk::Label = builder.get_object("count_lbl").unwrap();
    let count_progress: gtk::ProgressBar = builder.get_object("count_progress").unwrap();

    let adj: gtk::Adjustment = builder.get_object("timer").unwrap();
    let max = Arc::new(Mutex::new(adj.get_value()));
    let max_clone = Arc::clone(&max);

    adj.connect_value_changed(clone!(@strong sender => move |adj| {
        sender.send(Message::MaxAdjusted(adj.get_value())).expect("Cannot send Message");
    }));

    let elapsed = Arc::new(Mutex::new(0.));
    let reset_btn: gtk::Button = builder.get_object("reset_btn").unwrap();
    let elapsed_clone = Arc::clone(&elapsed);
    reset_btn.connect_clicked(move |_btn| {
        let mut val = elapsed_clone.lock().unwrap();
        *val = 0.;
    });

    let sender_clone = sender.clone();
    let timer = Timer::new();
    let elapsed_clone2 = elapsed.clone();
    let _guard = timer.schedule_repeating(chrono::Duration::milliseconds(100), move || {
        let mut val = elapsed_clone2.lock().unwrap();
        let max = max.lock().unwrap();
        if *val < *max {
            *val += 0.1;
            sender_clone.send(Message::CountFire(*val)).unwrap();
        }
    });

    receiver.attach(None, move |msg| {
        match msg {
            Message::CountFire(value) => {
                count_lbl.set_text(&format!("{:.1}", value));
                count_progress.set_fraction(value / *max_clone.lock().unwrap());
            }
            Message::MaxAdjusted(new_max) => {
                *max_clone.lock().unwrap() = new_max;
                let mut val = elapsed.lock().unwrap();
                if *val > new_max {
                    *val = new_max - 0.1;
                }
            }
        }
        glib::Continue(true)
    });

    gtk::main();
    Ok(())
}
