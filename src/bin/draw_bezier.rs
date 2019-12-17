use glium::{
    glutin::{
        self,
        dpi::LogicalPosition,
        event::{ElementState, MouseButton},
        event::{Event, StartCause, WindowEvent},
    },
    Surface,
};

use std::time::{Duration, Instant};

use toryn::bezier::Bezier;
use toryn::create_window;
use toryn::vertex::{draw_vertex_as_lines, draw_vertex_as_points, Vertex};

fn main() {
    let (event_loop, display) = create_window!(
        title: "Draw bezier curves",
        width: 500.,
        height: 500.,
    );

    let mut bezier_curve = Bezier::new().with_steps(100);
    let mut points: Vec<Vertex> = Vec::new();

    let mut last_pos = LogicalPosition::new(0., 0.);

    event_loop.run(move |event, _, control_flow| {
        // 60 FPS
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                WindowEvent::MouseInput { button, state, .. } => match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => {
                        let vertex = Vertex::new(last_pos.x as f32, last_pos.y as f32);
                        bezier_curve.push_point(vertex);
                        points.push(vertex);
                    }

                    (_, _) => {}
                },

                WindowEvent::CursorMoved { position, .. } => {
                    last_pos = position;
                }
                _ => {}
            },

            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => {
                    let mut frame = display.draw();
                    frame.clear_color(0., 0., 0., 1.);

                    if let Some(vertex) = bezier_curve.interpolate() {
                        draw_vertex_as_lines(&vertex, &display, &mut frame);
                    }

                    draw_vertex_as_points(&points, &display, &mut frame);

                    frame.finish().expect("Failed to swap buffers");
                }

                StartCause::Init => {}
                _ => {}
            },

            _ => {}
        }
    });
}
