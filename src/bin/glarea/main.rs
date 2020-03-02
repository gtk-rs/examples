//! # GLArea Sample
//!
//! This sample demonstrates how to use the GLArea widget with glium to
//! manage the rendering.

extern crate gio;
extern crate glib;
extern crate glium;
extern crate gtk;
extern crate shared_library;

use gio::prelude::*;
use glium::Surface;
use gtk::prelude::*;
use gtk::WidgetExt;

mod glium_backend;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

glium::implement_vertex!(Vertex, position, color);

struct TriangleData {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    program: glium::program::Program,
}

struct Renderer {
    triangle: Option<TriangleData>,
}

impl glium_backend::GliumRenderer for Renderer {
    fn initialize(&mut self, facade: &glium_backend::Facade, _gl_area: &gtk::GLArea) {
        let vertices = vec![
            Vertex {
                position: [0.0, 0.5],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = match glium::VertexBuffer::new(facade, &vertices) {
            Ok(buff) => buff,
            Err(err) => {
                eprintln!("Error creating vertex buffer: {:?}", err);
                return;
            }
        };
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vert_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;

        out vec3 vertex_color;

        void main() {
            vertex_color = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }"#;

        let frag_shader_src = r#"
        #version 140

        in vec3 vertex_color;

        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }"#;

        let program =
            match glium::Program::from_source(facade, vert_shader_src, frag_shader_src, None) {
                Ok(buff) => buff,
                Err(err) => {
                    eprintln!("Error creating shader program: {:?}", err);
                    return;
                }
            };

        self.triangle = Some(TriangleData {
            vertex_buffer,
            indices,
            program,
        });
    }

    fn tear_down(&mut self, _gl_area: &gtk::GLArea) {
        self.triangle = None;
    }

    fn draw(
        &mut self,
        frame: &mut glium::Frame,
        _gl_area: &gtk::GLArea,
        _gl_context: &gdk::GLContext,
    ) -> gtk::Inhibit {
        if let Some(triangle) = &self.triangle {
            frame.clear_color(0.3, 0.3, 0.3, 1.0);
            match frame.draw(
                &triangle.vertex_buffer,
                &triangle.indices,
                &triangle.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            ) {
                Err(err) => {
                    eprintln!("Error drawing frame: {:?}", err);
                    return Default::default();
                }
                _ => (),
            }
        }
        Inhibit(false)
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("GLArea example");
    window.set_size_request(400, 400);

    let gl_area = gtk::GLArea::new();

    let renderer = Renderer { triangle: None };
    glium_backend::hook_to_renderer(renderer, &gl_area, false);

    window.add(&gl_area);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.glarea"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
