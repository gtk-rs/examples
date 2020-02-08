/*
Example on how to use a communication thread alongside with the GUI thread.

Tricks used here:
- Use a seperate thread to handle incoming data and put it into a queue.
- Use a queue to show data on the GUI.
*/

extern crate futures;
extern crate gio;
extern crate glib;
extern crate gtk;
use futures::StreamExt;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};
use std::env::args;
use std::thread;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.communication_thread"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let label = Label::new(Some("Bla"));
    window.add(&label);

    // Create a channel between communication thread and main event loop:
    let (mut sender, mut receiver) = futures::channel::mpsc::channel::<String>(1000);

    // Spawn queue receive task on the main event loop:
    let main_context = glib::MainContext::default();
    let future = async move {
        while let Some(item) = receiver.next().await {
            label.set_text(&item);
        }
    };
    main_context.spawn_local(future);

    // Spawn seperate thread to handle communication:
    thread::spawn(move || {
        let mut counter = 0;
        loop {
            // Instead of a counter, your application code will
            // block here on TCP or serial communications.
            let data = format!("Counter = {}!", counter);
            println!("Thread received data: {}", data);
            sender.try_send(data).unwrap();
            counter += 1;
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    window.show_all();
}
