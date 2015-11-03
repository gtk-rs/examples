//! # GLArea Sample
//!
//! This sample demonstrates how to use GLAreas and OpenGL

extern crate gtk;
extern crate libc;

#[cfg(feature = "opengl")]
mod example {
    extern crate gl;
    extern crate glutin;
    extern crate shared_library;

    use gtk;
    use gtk::traits::*;
    use gtk::signal::Inhibit;
    use gtk::{GLArea, Window};
    use libc;
    use self::shared_library::dynamic_library::DynamicLibrary;
    use std::ptr;

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }

        let window = Window::new(gtk::WindowType::Toplevel).unwrap();
        let glarea = GLArea::new().unwrap();

        // Loads OpenGL addresses from libepoxy, looks up the corresponding epoxy symbol and
        // extracts the correct function address from epoxy's dispatch table.
        gl::load_with(|s| {
            let symbol = format!("epoxy_{}", s);
            unsafe {
                match DynamicLibrary::open(None).unwrap().symbol(&*symbol) {
                    Ok(v) => *(v as *const *const libc::c_void),
                    Err(_) => ptr::null(),
                }
            }
        });

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

