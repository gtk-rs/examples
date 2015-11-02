//! # GLArea Sample
//!
//! This sample demonstrates how to use GLAreas and OpenGL

extern crate gtk;

#[cfg(feature = "opengl")]
mod example {
    extern crate gl;
    extern crate glutin;

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

        // OpenGL/the gl crate needs a loader function to resolve OpenGL functions at runtime,
        // glutin provides this functionality in a relatively platform-independent way but requires
        // construction of a window/context first. To accomodate this, we create an invisible
        // window and use its get_proc_address function from gl to resolve missing functions.
        // This should work on any platform glutin works on, however the construction of the
        // separate window is unnecessary and it should be possible to isolate the platform
        // detection and get_proc_address functionality either in glutin or elsewhere.
        let dummy_win = glutin::WindowBuilder::new().with_visibility(false).build().unwrap();
        gl::load_with(|s| dummy_win.get_proc_address(s));

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        glarea.connect_render(|_, _| {
            unsafe {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
    
                gl::Flush();
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

