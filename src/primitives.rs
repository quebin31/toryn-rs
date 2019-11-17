pub mod vertex {
    use glium::implement_vertex;
    use glium::index::{NoIndices, PrimitiveType};
    use glium::uniforms::EmptyUniforms;
    use glium::{Display, Frame, Program, Surface, VertexBuffer};
    use lazy_static::lazy_static;

    #[derive(Debug, Clone, Copy)]
    pub struct Vertex {
        pub position: [f32; 2],
    }

    impl Vertex {
        pub fn new(x: f32, y: f32) -> Self {
            Self { position: [x, y] }
        }
    }

    implement_vertex!(Vertex, position);

    #[derive(Debug, Clone, Copy)]
    pub struct Vertex3 {
        pub position: [f32; 3],
    }

    impl Vertex3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self {
                position: [x, y, z],
            }
        }
    }

    implement_vertex!(Vertex3, position);

    pub fn draw_vertex_as_points(vertexs: &[Vertex], display: &Display, frame: &mut Frame) {
        lazy_static! {
            static ref INDICES: NoIndices = NoIndices(PrimitiveType::Points);
            static ref VERTEX_SHADER_SRC: &'static str = r#"
            La wea con                #version 330 core
                in vec2 position;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#;
            static ref FRAGMENT_SHADER_SRC: &'static str = r#"
                #version 330 core
                out vec4 color;
                void main() {
                    color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            "#;
        }

        let buffer = VertexBuffer::new(display, vertexs).unwrap();
        let program =
            Program::from_source(display, *VERTEX_SHADER_SRC, *FRAGMENT_SHADER_SRC, None).unwrap();

        frame
            .draw(
                &buffer,
                &*INDICES,
                &program,
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

pub mod points {
    use super::vertex::Vertex;
    use glium::Display;

    #[derive(Debug, Clone)]
    pub struct Point2d {
        pub x: i32,
        pub y: i32,
    }

    impl Point2d {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn origin() -> Self {
            Self { x: 0, y: 0 }
        }

        pub fn rotate(&self, rotation_deg: f32) -> Self {
            let sin_rd = rotation_deg.sin();
            let cos_rd = rotation_deg.cos();

            let (x0, y0) = (self.x as f32, self.y as f32);

            Self {
                x: (x0 * cos_rd + y0 * -sin_rd).round() as i32,
                y: (x0 * sin_rd + y0 * cos_rd).round() as i32,
            }
        }

        pub fn translate_x(&self, dist: i32) -> Self {
            Self {
                x: self.x + dist,
                y: self.y,
            }
        }

        pub fn translate_y(&self, dist: i32) -> Self {
            Self {
                x: self.x,
                y: self.y + dist,
            }
        }

        pub fn translate(&self, dist: i32) -> Self {
            Self {
                x: self.x + dist,
                y: self.y + dist,
            }
        }

        pub fn scale_x(&self, scale: f32) -> Self {
            Self {
                x: (self.x as f32 * scale).round() as i32,
                y: self.y,
            }
        }

        pub fn scale_y(&self, scale: f32) -> Self {
            Self {
                x: self.x,
                y: (self.y as f32 * scale).round() as i32,
            }
        }

        pub fn scale(&self, scale: f32) -> Self {
            Self {
                x: (self.x as f32 * scale).round() as i32,
                y: (self.y as f32 * scale).round() as i32,
            }
        }

        pub fn to_vertex(&self, display: &Display) -> Vertex {
            let inner_size = display.gl_window().window().inner_size();
            let x = 2.0 * self.x as f32 / inner_size.width as f32; // - 1.0;
            let y = 2.0 * self.y as f32 / inner_size.height as f32; // - 1.0;
            Vertex::new(x, y)
        }
    }
}

pub mod shapes {
    use super::points::Point2d;
    use super::vertex::draw_vertex_as_points;
    use glium::{Display, Frame};

    #[derive(Debug, Clone)]
    pub struct Line2d {
        beg_point: Point2d,
        end_point: Point2d,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LineDrawMethod {
        Incremental,
        MiddlePoint,
    }

    impl Line2d {
        pub fn new(begin: Point2d, end: Point2d) -> Self {
            Self {
                beg_point: begin,
                end_point: end,
            }
        }

        pub fn rotated(begin: Point2d, end: Point2d, rotation_deg: f32) -> Self {
            let mut slf = Self::new(begin, end);
            slf.rotate(rotation_deg);
            slf
        }

        pub fn slope(&self) -> f32 {
            let dy = (self.end_point.y - self.beg_point.y) as f32;
            let dx = (self.end_point.x - self.beg_point.x) as f32;
            dy / dx
        }

        pub fn rotate(&mut self, rotation_deg: f32) -> &mut Self {
            self.beg_point = self.beg_point.rotate(rotation_deg);
            self.end_point = self.end_point.rotate(rotation_deg);
            self
        }

        pub fn translate_x(&mut self, dist: i32) -> &mut Self {
            self.beg_point = self.beg_point.translate_x(dist);
            self.end_point = self.end_point.translate_x(dist);
            self
        }

        pub fn translate_y(&mut self, dist: i32) -> &mut Self {
            self.beg_point = self.beg_point.translate_y(dist);
            self.end_point = self.end_point.translate_y(dist);
            self
        }

        pub fn translate(&mut self, dist: i32) -> &mut Self {
            self.translate_x(dist).translate_y(dist)
        }

        pub fn scale_x(&mut self, scale: f32) -> &mut Self {
            self.beg_point = self.beg_point.scale_x(scale);
            self.end_point = self.end_point.scale_x(scale);
            self
        }

        pub fn scale_y(&mut self, scale: f32) -> &mut Self {
            self.beg_point = self.beg_point.scale_y(scale);
            self.end_point = self.end_point.scale_y(scale);
            self
        }

        pub fn scale(&mut self, scale: f32) -> &mut Self {
            self.scale_x(scale).scale_y(scale)
        }

        pub fn draw(&self, display: &Display, frame: &mut Frame, method: LineDrawMethod) {
            match method {
                LineDrawMethod::Incremental => self.incremental_draw(display, frame),
                LineDrawMethod::MiddlePoint => self.middle_point_draw(display, frame),
            }
        }

        fn incremental_draw(&self, display: &Display, frame: &mut Frame) {
            let slope =
                (self.end_point.y - self.beg_point.y) / (self.end_point.x - self.beg_point.x);

            let func = |x: i32| self.beg_point.y + slope * (x - self.beg_point.x);
            let steps = (self.end_point.x - self.beg_point.x).abs() as usize;

            let mut buffer = Vec::with_capacity(steps);
            let mut x = self.beg_point.x;
            for _ in 0..=steps {
                let y = func(x);
                buffer.push(Point2d::new(x, y).to_vertex(display));
                x += 1;
            }

            draw_vertex_as_points(&buffer, display, frame);
        }

        fn middle_point_draw(&self, display: &Display, frame: &mut Frame) {
            let ((x0, y0), (x1, y1)) = (
                (self.beg_point.x, self.beg_point.y),
                (self.end_point.x, self.end_point.y),
            );

            let slope = self.slope().abs();

            // Use axis 'y' as axis 'x', and viceversa
            // As if we were rotating the entire space
            let ((x0, y0), (x1, y1)) = if slope > 1.0 {
                ((y0, x0), (y1, x1))
            } else {
                ((x0, y0), (x1, y1))
            };

            // Accomode the relative x, to increase its value
            let ((x0, y0), (x1, y1)) = if x0 > x1 {
                ((x1, y1), (x0, y0))
            } else {
                ((x0, y0), (x1, y1))
            };

            let inc_y = if y0 >= y1 { -1 } else { 1 };

            let dx = x1 - x0;
            let dy = y1 - y0;

            let steps = (x1 - x0).abs() as usize;
            let mut buffer = Vec::with_capacity(steps);

            let mut x = x0;
            let mut y = y0;
            let mut d = 2 * dy - inc_y * dx;

            let inc_e = 2 * dy;
            let inc_ne = 2 * (dy - inc_y * dx);

            while x <= x1 {
                // If the space "was rotated", inverse the rotation and push
                // to draw buffer
                if slope > 1.0 {
                    buffer.push(Point2d::new(y, x).to_vertex(display));
                } else {
                    buffer.push(Point2d::new(x, y).to_vertex(display));
                }

                if inc_y * d <= 0 {
                    d += inc_e;
                    x += 1;
                } else {
                    d += inc_ne;
                    x += 1;
                    y += inc_y;
                }
            }

            draw_vertex_as_points(&buffer, display, frame);
        }
    }

    #[derive(Debug, Clone)]
    pub struct Shape2d {
        points: Vec<Point2d>,
    }

    impl Shape2d {
        pub fn new(points: &[Point2d]) -> Self {
            Self {
                points: Vec::from(points),
            }
        }

        pub fn rotated(points: &[Point2d], rotation_deg: f32) -> Self {
            let mut slf = Self::new(points);
            slf.rotate(rotation_deg);
            slf
        }

        pub fn add_point(&mut self, point: &Point2d) {
            self.points.push(point.clone());
        }

        pub fn rotate(&mut self, rotation_deg: f32) -> &mut Self {
            for point in &mut self.points {
                *point = point.rotate(rotation_deg);
            }
            self
        }

        pub fn translate_x(&mut self, dist: i32) -> &mut Self {
            for point in &mut self.points {
                *point = point.translate_x(dist);
            }
            self
        }

        pub fn translate_y(&mut self, dist: i32) -> &mut Self {
            for point in &mut self.points {
                *point = point.translate_y(dist);
            }
            self
        }

        pub fn translate(&mut self, dist: i32) -> &mut Self {
            for point in &mut self.points {
                *point = point.translate(dist);
            }
            self
        }

        pub fn scale_x(&mut self, scale: f32) -> &mut Self {
            for point in &mut self.points {
                *point = point.scale_x(scale);
            }
            self
        }

        pub fn scale_y(&mut self, scale: f32) -> &mut Self {
            for point in &mut self.points {
                *point = point.scale_y(scale);
            }
            self
        }

        pub fn scale(&mut self, scale: f32) -> &mut Self {
            for point in &mut self.points {
                *point = point.scale(scale);
            }
            self
        }

        pub fn draw(&self, display: &Display, frame: &mut Frame, method: LineDrawMethod) {
            if self.points.len() < 3 {
                return;
            }

            for i in 0..self.points.len() {
                let line = Line2d::new(
                    self.points[i].clone(),
                    self.points[(i + 1) % self.points.len()].clone(),
                );
                line.draw(display, frame, method);
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Circle2d {
        origin: Point2d,
        radius: u32,
    }

    impl Circle2d {
        pub fn new(origin: Point2d, radius: u32) -> Self {
            Self { origin, radius }
        }

        pub fn draw(&self, display: &Display, frame: &mut Frame) {
            let steps = (self.origin.y - self.origin.x) as usize;
            let mut buffer = Vec::with_capacity(steps);

            let x_o = self.origin.x;
            let y_o = self.origin.y;

            let mut push_points = |x: i32, y: i32| {
                buffer.push(Point2d::new(x, y).to_vertex(display));
                buffer.push(Point2d::new(x, 2 * y_o - y).to_vertex(display));
                buffer.push(Point2d::new(2 * x_o - x, y).to_vertex(display));
                buffer.push(Point2d::new(2 * x_o - x, 2 * y_o - y).to_vertex(display));
                buffer.push(Point2d::new(y, x).to_vertex(display));
                buffer.push(Point2d::new(y, 2 * x_o - x).to_vertex(display));
                buffer.push(Point2d::new(2 * y_o - y, x).to_vertex(display));
                buffer.push(Point2d::new(2 * y_o - y, 2 * x_o - x).to_vertex(display));
            };

            let r = self.radius as i32;
            let mut x = self.origin.x;
            let mut y = self.origin.y + r;
            let mut d = 1 - r;

            while y >= x {
                //println!("{:?}", (x, y));
                push_points(x, y);
                if d < 0 {
                    d += 2 * (x - x_o) + 3;
                    x += 1;
                } else {
                    d += 2 * (x - x_o - y + y_o) + 5;
                    x += 1;
                    y -= 1;
                }
            }

            draw_vertex_as_points(&buffer, display, frame);
        }
    }
}
