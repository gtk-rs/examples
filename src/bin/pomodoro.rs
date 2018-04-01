//! # Pomodoro Sample
//!
//! This sample demonstrates how to create a basic Pomodoro timer application
//! with shared application state that callbacks access and modify.
//! The Rust version of this GTK application is a port of github.com/ejmg/tomaty

extern crate gtk;
extern crate pango;
extern crate chrono;

#[cfg(feature = "gtk_3_12")]
mod pomodoro {
    use gtk;
    
    use chrono::Duration;

    use std::rc::Rc;
    use std::cell::RefCell;

    use gtk::prelude::*;

    macro_rules! TIMER_FRMT {() => (r###"
    <span font='34'>{}</span>
    "###)}

    macro_rules! COUNT {() => (r###"
    <span font='11'><tt>Tomatoros Completed: {}</tt></span>"###)}

    macro_rules! TOTAL_TIME {() => (r###"
    <span font='11'><tt>Total Time: {} minutes</tt></span>"###)}

    const TOMA_MINUTES: i64 = 3;
    const BREAK_MINUTES: i64 = 5;

    const TOMA_MSG: &str = r###"
    <span font='16'>Tomatoro Done!
    Start Break?</span>"###;

    const BREAK_MSG: &str = r###"
    <span font='16'>Break Over!
    Start Tomatoro?</span>"###;

    const TOMA_RESTART_MSG: &str = r###"
    <span font='16'>Start Tomatoro?</span>"###;

    const BREAK_RESTART_MSG: &str = r###"
    <span font='16'>Start Break?</span>"###;

    fn make_label(label: &str) -> gtk::Label {
        let new_label = gtk::Label::new(label);
        new_label.set_margin_start(0);
        new_label.set_margin_end(0);
        new_label.set_margin_top(0);
        new_label.set_margin_bottom(0);
        new_label.set_justify(gtk::Justification::Center);
        new_label
    }

    fn make_tomaty_notebook() -> gtk::Notebook {
        let new_notebook = gtk::Notebook::new();
        new_notebook.set_size_request(250, 150);
        new_notebook
    }

    fn make_tomaty_page() -> gtk::Box {
        let new_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        new_box.set_homogeneous(false);
        new_box
    }

    fn make_button(margin_top: i32, margin_bottom: i32) -> gtk::Button {
        let new_button = gtk::Button::new();
        new_button.set_label("start");
        new_button.set_margin_start(0);
        new_button.set_margin_end(0);
        new_button.set_margin_top(margin_top);
        new_button.set_margin_bottom(margin_bottom);
        new_button.set_halign(gtk::Align::Center);
        new_button
    }

    struct Tomaty {
        tomatos_completed: i32,
        running: bool,
        break_period: bool,
        toma_time: Duration,
        break_time: Duration,
        remaining_time: Duration,
        tomatoro_length: Duration,
        tomaty_button: gtk::Button,
        timer_label: gtk::Label,
        count_label: gtk::Label,
        total_label: gtk::Label,
    }

    fn update_timer(tomtom: &mut Tomaty) {
        let minutes = tomtom.remaining_time.num_minutes();
        let seconds = tomtom.remaining_time.num_seconds() % 60;

        let timer_formatted =
            format!(TIMER_FRMT!(), format!("{:02}:{:02}", minutes, seconds));
        tomtom.timer_label.set_markup(&timer_formatted);
    }

    fn connect_click_start(tomaty: Rc<RefCell<Tomaty>>) {
        let outer_tomato_heaven = tomaty.clone();
        let button = &outer_tomato_heaven.borrow().tomaty_button;

        button.connect_clicked(move |cb_button: &gtk::Button| {
            let mut tomtom = tomaty.borrow_mut();
            if tomtom.running {
                tomtom.running = false;
                update_button(cb_button);
                if tomtom.break_period {
                    tomtom.timer_label.set_markup(BREAK_RESTART_MSG);
                    tomtom.remaining_time = tomtom.break_time;
                } else {
                    tomtom.timer_label.set_markup(TOMA_RESTART_MSG);
                    tomtom.remaining_time = tomtom.toma_time;
                };
            } else {
                tomtom.running = true;
                update_button(cb_button);
                if tomtom.break_period {
                    tomtom.remaining_time = tomtom.break_time;
                } else {
                    tomtom.remaining_time = tomtom.toma_time;
                };
                update_timer(&mut tomtom);
                add_timeout_countdown(tomaty.clone());
            };
            println!("Button clicked!");
        });

    }

    fn alarm() {
        println!("POMODORO OVER, TAKE A BREAK!");
    }

    fn add_timeout_countdown(tomaty: Rc<RefCell<Tomaty>>) {
        gtk::timeout_add_seconds(1, move || {
            let mut tomtom = tomaty.borrow_mut();
            if tomtom.remaining_time == Duration::seconds(0) {
                alarm();
                tomtom.running = false;
                update_button(&tomtom.tomaty_button);
                if tomtom.break_period {
                    tomtom.timer_label.set_markup(BREAK_MSG);
                    tomtom.break_period = false;
                } else {
                    tomtom.tomatos_completed += 1;
                    let count_formatted =
                        format!(COUNT!(), tomtom.tomatos_completed);
                    tomtom.count_label.set_markup(&count_formatted);
                    let total = tomtom.tomatoro_length * tomtom.tomatos_completed;
                    let total_formatted =
                        format!(TOTAL_TIME!(), total.num_minutes());
                    tomtom.total_label.set_markup(&total_formatted);
                    tomtom.timer_label.set_markup(TOMA_MSG);
                    tomtom.break_period = true;
                }
                return gtk::Continue(false)
            }
            if !tomtom.running {
                return gtk::Continue(false)
            }
            tomtom.remaining_time = tomtom.remaining_time - Duration::seconds(1);
            update_timer(&mut tomtom);
            gtk::Continue(true)
        });
    }

    fn update_button(button: &gtk::Button) {
        match button.get_label().as_ref().map(String::as_ref) {
            Some("start") => button.set_label("restart"),
            _ => button.set_label("start"),
        }
    }

    fn make_window() -> gtk::Window {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        window.set_title("tomaty: gtk::Focus");
        window.set_border_width(5);
        window.set_resizable(false);

        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        // create notebook, add as main and sole child widget of window
        let notebook = make_tomaty_notebook();
        window.add(&notebook);

        let timer_page = make_tomaty_page();
        let remaining_default = Duration::minutes(0);
        let timer_label = make_label("");
        timer_label.set_markup(TOMA_RESTART_MSG);
        timer_page.pack_start(&timer_label, true, true, 0);

        let tomaty_button = make_button(5, 5);
        timer_page.pack_start(&tomaty_button, false, false, 0);

        let tab_label = make_label("tomatoro");
        notebook.append_page(&timer_page, Some(&tab_label));

        let stats_page = make_tomaty_page();
        let tomatos_completed_default = 0;
        let count_label_formatted =
            format!(COUNT!(), tomatos_completed_default);
        let count_label = make_label("");
        count_label.set_markup(&count_label_formatted);
        count_label.set_margin_start(10);
        count_label.set_margin_end(10);

        let tomatoro_length_default = Duration::minutes(25);
        let total = tomatoro_length_default * tomatos_completed_default;
        let total_formatted=
            format!(TOTAL_TIME!(), total);

        let total_label = make_label("");
        total_label.set_markup(&total_formatted);
        total_label.set_margin_end(25);
        total_label.set_justify(gtk::Justification::Left);

        stats_page.pack_start(&count_label, false, false, 0);
        stats_page.pack_start(&total_label, false, false, 0);

        let stats_tab_label = make_label("stats");
        notebook.append_page(&stats_page, Some(&stats_tab_label));

        let tomaty = Rc::new(RefCell::new(Tomaty {
            tomatos_completed: tomatos_completed_default,
            running: false,
            break_period: false,
            toma_time: Duration::minutes(TOMA_MINUTES),
            break_time: Duration::minutes(BREAK_MINUTES),
            remaining_time: remaining_default,
            tomatoro_length: tomatoro_length_default,
            tomaty_button,
            timer_label,
            count_label,
            total_label,
        }));

        connect_click_start(tomaty.clone());
        window.show_all();
        window
    }

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
    }
        make_window();
        gtk::main();
    }
}

#[cfg(feature = "gtk_3_12")]
fn main() {
    pomodoro::main()
}

#[cfg(not(feature = "gtk_3_12"))]
fn main() {
    println!("This example requires GTK 3.12 or later");
    println!("Did you forget to build with `--features gtk_3_12`?");
}
