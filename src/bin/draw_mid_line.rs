use glium::glutin::{self, dpi::LogicalSize};
use glium::Surface;
use toryn::points::Point2d;
use toryn::shapes::{Line2d, LineDrawMethod};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Middle-point line")
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

        let lines = vec![
            Line2d::new(Point2d::new(0, 0), Point2d::new(100, 100)),
            Line2d::new(Point2d::new(0, 0), Point2d::new(-100, 100)),
            Line2d::new(Point2d::new(0, 0), Point2d::new(-100, -100)),
            Line2d::new(Point2d::new(0, 0), Point2d::new(100, -100)),
            Line2d::new(Point2d::new(-100, 100), Point2d::new(100, 100)),
            Line2d::new(Point2d::new(-100, -100), Point2d::new(100, -100)),
            Line2d::new(Point2d::new(-100, -100), Point2d::new(100, -100)),
            Line2d::new(Point2d::new(-100, -100), Point2d::new(100, -100)),
            Line2d::new(Point2d::new(-100, -100), Point2d::new(-100, 100)),
            Line2d::new(Point2d::new(100, 100), Point2d::new(100, -100)),
            Line2d::rotated(
                Point2d::new(0, 0),
                Point2d::new(100, 100),
                45_f32.to_radians(),
            ),
        ];

        let mut frame = display.draw();
        frame.clear_color(0., 0., 0., 1.);
        for line in lines {
            line.draw(&display, &mut frame, LineDrawMethod::MiddlePoint);
        }
        frame.finish().expect("Failed to swap buffers");
    });
}
