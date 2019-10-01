use glium::glutin::{self, dpi::LogicalSize};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::EmptyUniforms;
use glium::{Program, Surface, VertexBuffer};
use lazy_static::lazy_static;
use toryn::vertex::Vertex;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Lines")
        .with_inner_size(LogicalSize::new(300., 300.));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("Failed to create display");

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        lazy_static! {
            static ref INDICES: NoIndices = NoIndices(PrimitiveType::LineLoop);
            static ref VERTEX_SHADER_SRC: &'static str = r#"
                #version 140
                in vec2 position;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#;
            static ref FRAGMENT_SHADER_SRC: &'static str = r#"
                #version 140
                out vec4 color;
                void main() {
                    color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            "#;
        }

        let vertex = vec![Vertex::new(0., 0.), Vertex::new(0.3, 0.3)];
        let buffer = VertexBuffer::new(&display, &vertex).unwrap();
        let program =
            Program::from_source(&display, *VERTEX_SHADER_SRC, *FRAGMENT_SHADER_SRC, None).unwrap();

        let mut frame = display.draw();
        frame.clear_color(0., 0., 0., 1.);
        frame
            .draw(
                &buffer,
                &*INDICES,
                &program,
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().expect("Failed to swap buffers");
    });
}
