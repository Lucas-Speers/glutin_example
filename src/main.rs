use std::{fs::File, io::Read};

use glium::{glutin::surface::WindowSurface, winit::{self, application::ApplicationHandler, window::Window}, Program, Surface};

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct App {
    pub window: Window,
    pub display: glium::Display<WindowSurface>,
    pub t: f32,
    pub program: Program,
}

fn init_shaders(display: &glium::Display<WindowSurface>) -> Program {
    let mut vertex_shader_src = String::new();
    File::open("shader.vert")
        .unwrap()
        .read_to_string(&mut vertex_shader_src)
        .unwrap();

    let mut fragment_shader_src = String::new();
    File::open("shader.frag")
        .unwrap()
        .read_to_string(&mut fragment_shader_src)
        .unwrap();

    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}
impl App {
    fn draw(&mut self) {
        let mut target = self.display.draw();

        self.t += 0.1;
        
        let shape = vec![
            Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] }
        ];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            matrix: [
                [ self.t.cos(), self.t.sin(), 0.0, 0.0],
                [-self.t.sin(), self.t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &self.program, &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => self.draw(),
            winit::event::WindowEvent::Resized(window_size) => self.display.resize(window_size.into()),
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.request_redraw();
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().expect("Could not build event loop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Rust with Gluim go brrrrrrrrr").build(&event_loop);

    let program = init_shaders(&display);
    
    let mut app = App { window, display, t: 0.0, program};
    event_loop.run_app(&mut app).expect("event loop failed to run app");
}