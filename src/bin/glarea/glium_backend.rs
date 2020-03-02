//! # glium_backend utility module
//!
//! This module provides some general utility types that allow glium to manage
//! the rendering of a GLArea.

use std::cell::RefCell;
use std::rc::Rc;

use gtk::{GLAreaExt, WidgetExt};
use shared_library::dynamic_library::DynamicLibrary;

/// The primary trait of this module, implemented by users to facilitate
/// glium rendering for their GLArea
pub trait GliumRenderer {
    /// Use this method to set up your GL resources. This is called during the
    /// `realize` signal callback for the GLArea. At this point, a valid
    /// OpenGL context for the GLArea has been set up and is available via
    /// `Facade`.
    fn initialize(&mut self, facade: &Facade, gl_area: &gtk::GLArea);

    /// Use this method to tear down your GL resources. This is called during
    /// the `unrealize` signal callback for the GLArea.
    fn tear_down(&mut self, gl_area: &gtk::GLArea);

    /// Use this method to draw to the given Frame. This is called during the
    /// `render` signal callback for the GLArea.
    fn draw(
        &mut self,
        frame: glium::Frame,
        gl_area: &gtk::GLArea,
        gl_context: &gdk::GLContext,
    ) -> gtk::Inhibit;
}

/// Utility function showing how to use a GlAreaGliumHook to connect a GLArea to
/// glium via a GliumRenderer.
pub fn hook_to_renderer<R>(renderer: R, gl_area: &gtk::GLArea, check_current_context: bool)
where
    R: GliumRenderer + 'static,
{
    let hook = GlAreaGliumHook::new(renderer, check_current_context);

    gl_area.connect_realize(glib::clone!(@weak hook => move |gl_area| {
        hook.borrow_mut().realize(gl_area);
    }));

    gl_area.connect_unrealize(glib::clone!(@weak hook => move |gl_area| {
        hook.borrow_mut().unrealize(gl_area);
    }));

    gl_area
        .connect_render(move |gl_area, gl_context| hook.borrow_mut().render(gl_area, gl_context));
}

/// This struct is used to hook a GLArea widget to glium using a GliumRenderer
pub struct GlAreaGliumHook<R>
where
    R: GliumRenderer,
{
    renderer: R,
    facade: Option<Facade>,
    check_current_context: bool,
}

impl<R> GlAreaGliumHook<R>
where
    R: GliumRenderer,
{
    pub fn new(renderer: R, check_current_context: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            renderer,
            facade: None,
            check_current_context,
        }))
    }

    pub fn realize(&mut self, gl_area: &gtk::GLArea) {
        let context = unsafe {
            glium::backend::Context::new::<_>(
                Backend::new(gl_area.clone()),
                self.check_current_context,
                Default::default(),
            )
            .unwrap() // TODO: Better error handling`
        };

        let facade = Facade { context };
        self.renderer.initialize(&facade, gl_area);
        self.facade = Some(facade);
    }

    pub fn unrealize(&mut self, gl_area: &gtk::GLArea) {
        self.renderer.tear_down(gl_area);
        self.facade = None;
    }

    pub fn render(&mut self, gl_area: &gtk::GLArea, gl_context: &gdk::GLContext) -> gtk::Inhibit {
        match &self.facade {
            Some(facade) => {
                let frame = glium::Frame::new(
                    facade.context.clone(),
                    facade.context.get_framebuffer_dimensions(),
                );
                self.renderer.draw(frame, gl_area, gl_context)
            }
            None => Default::default(),
        }
    }
}

pub struct Facade {
    context: Rc<glium::backend::Context>,
}

impl glium::backend::Facade for Facade {
    fn get_context(&self) -> &Rc<glium::backend::Context> {
        &self.context
    }
}

struct Backend {
    gl_area: gtk::GLArea,
}

impl Backend {
    pub fn new(gl_area: gtk::GLArea) -> Self {
        Self { gl_area }
    }
}

unsafe impl glium::backend::Backend for Backend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::os::raw::c_void {
        // Strictly speaking, this call should only be needed once, but putting
        // it here ensures the correct behaviour even if it is a bit slow at
        // startup.
        epoxy::load_with(
            |symbol| match DynamicLibrary::open(None).unwrap().symbol(symbol) {
                Ok(pointer) => pointer,
                Err(_) => std::ptr::null(),
            },
        );
        epoxy::get_proc_addr(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        (
            self.gl_area.get_allocated_width() as u32,
            self.gl_area.get_allocated_height() as u32,
        )
    }

    fn is_current(&self) -> bool {
        match self.gl_area.get_context() {
            None => false,
            Some(context) => match gdk::GLContext::get_current() {
                None => false,
                Some(current_context) => context == current_context,
            },
        }
    }

    unsafe fn make_current(&self) {
        self.gl_area.make_current();
    }
}
