//! # GLArea Sample
//!
//! This sample demonstrates how to use GLAreas and OpenGL

extern crate gtk;
extern crate libc;

#[cfg(feature = "opengl")]
mod example {
    #[link(name = "epoxy")] extern {}

    mod epoxy {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    use gtk;
    use gtk::traits::*;
    use gtk::signal::Inhibit;
    use gtk::{GLArea, Window};

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }

        let window = Window::new(gtk::WindowType::Toplevel).unwrap();
        let glarea = GLArea::new().unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        glarea.connect_render(|_, _| {
            unsafe {
                epoxy::Gl.ClearColor(1.0, 0.0, 0.0, 1.0);
                epoxy::Gl.Clear(epoxy::COLOR_BUFFER_BIT);

                epoxy::Gl.Flush();
            };

            Inhibit(false)
        });

        window.set_title("GLArea Example");
        window.set_default_size(400, 400);
        window.add(&glarea);

        window.show_all();
        gtk::main();
    }
}

#[cfg(feature = "opengl")]
fn main() {
    example::main()
}

#[cfg(not(feature = "opengl"))]
fn main() {
    println!("Did you forget to build with `--features opengl`?");
}
