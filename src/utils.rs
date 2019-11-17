#[macro_export]
macro_rules! create_window {
    (title: $title:expr, width: $width:expr, height: $height:expr,) => {{
        use glium::glutin::{self, dpi::LogicalSize};

        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title($title)
            .with_inner_size(LogicalSize::new($width, $height));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).expect("Failed to create display");

        (event_loop, display)
    }};
}
