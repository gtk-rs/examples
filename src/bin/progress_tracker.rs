//! Track progress with a background thread and a channel.

#[macro_use]
extern crate cascade;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::rc::Rc;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn main() {
    let application = gtk::Application::new(
        "com.github.progress-tracker",
        gio::ApplicationFlags::empty(),
    ).expect("initialization failed");

    application.connect_startup(|app| {
        Application::new(app);
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

pub struct Application {
    pub widgets: Rc<Widgets>,
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let app = Application {
            widgets: Rc::new(Widgets::new(app)),
        };

        app.connect_progress();

        app
    }

    fn connect_progress(&self) {
        let widgets = self.widgets.clone();
        self.widgets.main_view.button.connect_clicked(move |_| {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                for v in 1..=10 {
                    tx.send(v);
                    thread::sleep(Duration::from_millis(500));
                }
            });

            let widgets = widgets.clone();
            gtk::timeout_add(16, move || match rx.try_recv() {
                Ok(value) => {
                    widgets
                        .main_view
                        .progress
                        .set_fraction(f64::from(value) / 10.0);

                    if value == 10 {
                        widgets
                            .view_stack
                            .set_visible_child(&widgets.complete_view.container);
                    }

                    gtk::Continue(true)
                }
                Err(TryRecvError::Empty) => gtk::Continue(true),
                Err(TryRecvError::Disconnected) => gtk::Continue(false),
            });
        });
    }
}

pub struct Widgets {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub view_stack: gtk::Stack,
    pub main_view: MainView,
    pub complete_view: CompleteView,
}

impl Widgets {
    pub fn new(application: &gtk::Application) -> Self {
        let complete_view = CompleteView::new();
        let main_view = MainView::new();

        let view_stack = cascade! {
            gtk::Stack::new();
            ..set_border_width(6);
            ..set_vexpand(true);
            ..set_hexpand(true);
            ..add(&main_view.container);
            ..add(&complete_view.container);
        };

        let header = Header::new();

        let window = cascade! {
            gtk::ApplicationWindow::new(application);
            ..set_icon_name("package-x-generic");
            ..set_property_window_position(gtk::WindowPosition::Center);
            ..set_titlebar(&header.container);
            ..add(&view_stack);
            ..show_all();
            ..set_default_size(500, 250);
            ..connect_delete_event(move |window, _| {
                window.destroy();
                Inhibit(false)
            });
        };

        Widgets {
            window,
            header,
            view_stack,
            main_view,
            complete_view,
        }
    }
}

pub struct Header {
    container: gtk::HeaderBar,
}

impl Header {
    pub fn new() -> Self {
        let container = cascade! {
            gtk::HeaderBar::new();
            ..set_title("Progress Tracker");
            ..set_show_close_button(true);
        };

        Header { container }
    }
}

pub struct CompleteView {
    pub container: gtk::Grid,
}

impl CompleteView {
    pub fn new() -> Self {
        let label = cascade! {
            gtk::Label::new(None);
            ..set_markup("Task complete");
            ..set_halign(gtk::Align::Center);
            ..set_valign(gtk::Align::Center);
            ..set_vexpand(true);
            ..set_hexpand(true);
        };

        let container = cascade! {
            gtk::Grid::new();
            ..set_vexpand(true);
            ..set_hexpand(true);
            ..add(&label);
        };

        CompleteView { container }
    }
}

pub struct MainView {
    pub container: gtk::Grid,
    pub progress: gtk::ProgressBar,
    pub button: gtk::Button,
}

impl MainView {
    pub fn new() -> Self {
        let progress = cascade! {
            gtk::ProgressBar::new();
            ..set_text("Progress Bar");
            ..set_show_text(true);
            ..set_hexpand(true);
        };

        let button = cascade! {
            gtk::Button::new();
            ..set_label("start");
            ..set_halign(gtk::Align::Center);
        };

        let container = cascade! {
            gtk::Grid::new();
            ..attach(&progress, 0, 0, 1, 1);
            ..attach(&button, 0, 1, 1, 1);
            ..set_row_spacing(6);
            ..set_border_width(6);
            ..set_vexpand(true);
            ..set_hexpand(true);
        };

        MainView {
            container,
            progress,
            button,
        }
    }
}
