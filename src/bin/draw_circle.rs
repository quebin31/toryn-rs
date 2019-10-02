use glium::glutin::{self, dpi::LogicalSize};
use glium::Surface;
use toryn::points::Point2d;
use toryn::shapes::{Circle2d, Line2d, LineDrawMethod};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Circle 2D")
        .with_inner_size(LogicalSize::new(500., 500.));
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

        let x_o = 30;
        let y_o = 30;
        let og_axis_x = Line2d::new(Point2d::new(-250, 0), Point2d::new(250, 0));
        let og_axis_y = Line2d::new(Point2d::new(0, -250), Point2d::new(0, 250));
        let axis_x = Line2d::new(Point2d::new(-250, y_o), Point2d::new(250, y_o));
        let axis_y = Line2d::new(Point2d::new(x_o, -250), Point2d::new(x_o, 250));
        let circle = Circle2d::new(Point2d::new(x_o, y_o), 50);

        let mut frame = display.draw();
        frame.clear_color(0., 0., 0., 1.);
        axis_x.draw(&display, &mut frame, LineDrawMethod::MiddlePoint);
        axis_y.draw(&display, &mut frame, LineDrawMethod::MiddlePoint);
        og_axis_x.draw(&display, &mut frame, LineDrawMethod::MiddlePoint);
        og_axis_y.draw(&display, &mut frame, LineDrawMethod::MiddlePoint);
        circle.draw(&display, &mut frame);
        frame.finish().expect("Failed to swap buffers");
    });
}
