use glium::{glutin, Surface};
use toryn::points::Point2d;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Draw point");
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

        let points = vec![
            Point2d::new(0.5, 0.5),
            Point2d::new(0.2, 0.5),
            Point2d::new(0.3, 0.5),
            Point2d::new(0.1, 0.5),
        ];

        let mut frame = display.draw();
        frame.clear_color(0., 0., 0., 1.);
        for point in points {
            point.paint(&display, &mut frame);
        }
        frame.finish().expect("Failed to swap buffers");
    });
}
