pub mod points {
    use glium::implement_vertex;
    use glium::index::{NoIndices, PrimitiveType};
    use glium::uniforms::EmptyUniforms;
    use glium::{Display, Frame, Program, Surface, VertexBuffer};
    use lazy_static::lazy_static;

    #[derive(Debug, Clone, Copy)]
    pub struct Point2d {
        position: [f32; 2],
    }

    implement_vertex!(Point2d, position);

    impl Point2d {
        pub fn new(x: f32, y: f32) -> Self {
            Self { position: [x, y] }
        }

        pub fn origin() -> Self {
            Self { position: [0., 0.] }
        }

        pub fn x(&mut self) -> &mut f32 {
            &mut self.position[0]
        }

        pub fn y(&mut self) -> &mut f32 {
            &mut self.position[1]
        }

        pub fn paint(self, display: &Display, frame: &mut Frame) {
            lazy_static! {
                static ref INDICES: NoIndices = NoIndices(PrimitiveType::Points);
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

            let point = vec![self];
            let buffer = VertexBuffer::new(display, &point).unwrap();

            let program =
                Program::from_source(display, *VERTEX_SHADER_SRC, *FRAGMENT_SHADER_SRC, None)
                    .unwrap();

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
}

pub mod shapes {
    use super::points::Point2d;

    #[derive(Debug, Clone)]
    pub struct Line2d {
        beg_point: Point2d,
        end_point: Point2d,
    }
}
