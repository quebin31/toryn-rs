use glium::{
    glutin::{
        self,
        event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent},
    },
    index::PrimitiveType,
    uniform,
    uniforms::{AsUniformValue, UniformValue},
    Display, DrawParameters, IndexBuffer, Program, Rect, Surface, VertexBuffer,
};

use glm::{
    cross,
    ext::{look_at, perspective, rotate},
    normalize, Mat4, Vec3, Vec4,
};

use lazy_static::lazy_static;
use std::time::{Duration, Instant};
use toryn::{create_window, primitives::vertex::Vertex3};

struct Settings {
    width: f64,
    height: f64,
    aspect: f32,
    z_far: f32,
    z_near: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    handle_motion: bool,
}

struct Camera {
    fov: f32,
    position: Vec3,
    front: Vec3,
    up: Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    axisx_init: bool,
    axisy_init: bool,
    axisx_value: f32,
    axisy_value: f32,
    use_perspective: bool,
}

struct Object {
    vertices: Vec<Vertex3>,
    indices: Vec<u32>,
}

struct GlmMat4(Mat4);

impl AsUniformValue for GlmMat4 {
    fn as_uniform_value(&self) -> UniformValue {
        let cols = self.0.as_array();
        let cols = [
            cols[0].as_array().to_owned(),
            cols[1].as_array().to_owned(),
            cols[2].as_array().to_owned(),
            cols[3].as_array().to_owned(),
        ];

        UniformValue::Mat4(cols)
    }
}

fn main() {
    let mut settings = Settings {
        width: 500.,
        height: 500.,
        aspect: 1.,
        z_far: 100.,
        z_near: 0.1,
        rot_x: (-55_f32).to_radians(),
        rot_y: 0.0,
        rot_z: 0.0,
        handle_motion: false,
    };

    let mut camera = Camera {
        fov: 45_f32.to_radians(),
        position: Vec3::new(0., 0., 3.),
        front: Vec3::new(0., 0., -1.),
        up: Vec3::new(0., 1., 0.),
        speed: 0.05,
        yaw: -90.,
        pitch: 0.,
        sensitivity: 0.1,
        axisx_init: false,
        axisy_init: false,
        axisx_value: 0.,
        axisy_value: 0.,
        use_perspective: true,
    };

    let (event_loop, display) = create_window!(
        title: "Camera Projection",
        width: settings.width,
        height: settings.height,
    );

    lazy_static! {
        static ref VERTEX_SHADER_SRC: &'static str = r#"
            #version 330 core
            in vec3 position;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * view * model * vec4(position, 1.0);
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

    let program =
        Program::from_source(&display, *VERTEX_SHADER_SRC, *FRAGMENT_SHADER_SRC, None).unwrap();

    let mut object = Object {
        vertices: vec![
            Vertex3::new(0.5, 0.5, 0.0),
            Vertex3::new(0.5, -0.5, 0.0),
            Vertex3::new(-0.5, -0.5, 0.0),
            Vertex3::new(-0.5, 0.5, 0.0),
        ],
        indices: vec![0_u32, 1, 3, 1, 2, 3],
    };

    let draw_parameters = DrawParameters {
        viewport: Some(Rect {
            left: 0,
            bottom: 0,
            width: settings.width as u32,
            height: settings.height as u32,
        }),
        ..Default::default()
    };

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

                WindowEvent::KeyboardInput { input, .. } => {
                    handle_input(&input, &mut settings, &mut camera, &mut object)
                }

                WindowEvent::CursorEntered { .. } => {
                    settings.handle_motion = true;
                }

                WindowEvent::AxisMotion { axis, value, .. } => {
                    if settings.handle_motion {
                        handle_motion(axis, value, &mut camera)
                    }
                }
                _ => {}
            },

            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => draw(
                    &camera,
                    &settings,
                    &display,
                    &object,
                    &program,
                    &draw_parameters,
                ),

                StartCause::Init => (),
                _ => {}
            },

            _ => {}
        }
    });
}

fn draw(
    camera: &Camera,
    settings: &Settings,
    display: &Display,
    object: &Object,
    program: &Program,
    draw_parameters: &DrawParameters,
) {
    let vertex_buffer =
        VertexBuffer::new(display, &object.vertices).expect("Failed to allocate vertex buffer");
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &object.indices)
        .expect("Failed to allocate index buffer");

    let one_matrix = Mat4::new(
        Vec4::new(1., 0., 0., 0.),
        Vec4::new(0., 1., 0., 0.),
        Vec4::new(0., 0., 1., 0.),
        Vec4::new(0., 0., 0., 1.),
    );

    let model_matrix = GlmMat4(rotate(
        &one_matrix,
        settings.rot_x,
        Vec3::new(1.0, 0.0, 0.0),
    ));

    let model_matrix =
        GlmMat4(model_matrix.0 * rotate(&one_matrix, settings.rot_y, Vec3::new(0.0, 1.0, 0.0)));

    let model_matrix =
        GlmMat4(model_matrix.0 * rotate(&one_matrix, settings.rot_z, Vec3::new(0.0, 0.0, 1.0)));

    let view_matrix = GlmMat4(look_at(
        camera.position,
        camera.position + camera.front,
        camera.up,
    ));

    let projection_matrix = if camera.use_perspective {
        GlmMat4(perspective(
            camera.fov,
            settings.aspect,
            settings.z_near,
            settings.z_far,
        ))
    } else {
        GlmMat4(ortho_matrix(
            0.,
            settings.width as f32,
            0.,
            settings.height as f32,
            settings.z_near,
            settings.z_far,
        ))
    };

    let uniforms = uniform! {
        model: model_matrix,
        view: view_matrix,
        projection: projection_matrix,
    };

    // Draw!
    let mut frame = display.draw();
    frame.clear_color(0., 0., 0., 1.);
    frame
        .draw(
            &vertex_buffer,
            &index_buffer,
            program,
            &uniforms,
            draw_parameters,
        )
        .expect("Failed to draw");
    frame.finish().expect("Failed to swap buffers");
}

fn handle_motion(axis: u32, value: f64, camera: &mut Camera) {
    if axis == 1 {
        if !camera.axisy_init {
            camera.axisy_value = value as f32;
            camera.axisy_init = true;
        }

        camera.pitch += (value as f32 - camera.axisy_value) * camera.sensitivity;
        camera.axisy_value = value as f32;
    } else {
        if !camera.axisx_init {
            camera.axisx_value = value as f32;
            camera.axisx_init = true;
        }

        camera.yaw += (camera.axisx_value - value as f32) * camera.sensitivity;
        camera.axisx_value = value as f32;
    }

    if camera.pitch > 89. {
        camera.pitch = 89.;
    } else if camera.pitch < -89. {
        camera.pitch = -89.;
    }

    let cos_pitch = camera.pitch.to_radians().cos();
    let sin_pitch = camera.pitch.to_radians().sin();
    let cos_yaw = camera.yaw.to_radians().cos();
    let sin_yaw = camera.yaw.to_radians().sin();

    let x = cos_yaw * cos_pitch;
    let y = sin_pitch;
    let z = sin_yaw * cos_pitch;
    camera.front = normalize(Vec3::new(x, y, z));
}

fn handle_input(
    input: &KeyboardInput,
    settings: &mut Settings,
    camera: &mut Camera,
    object: &mut Object,
) {
    match (input.state, input.virtual_keycode) {
        (ElementState::Pressed, Some(VirtualKeyCode::X)) => {
            settings.rot_x += 1_f32.to_radians();
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Y)) => {
            settings.rot_y += 1_f32.to_radians();
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Z)) => {
            settings.rot_z += 1_f32.to_radians();
        }

        (ElementState::Pressed, Some(VirtualKeyCode::W)) => {
            camera.position = camera.position + camera.front * camera.speed;
        }

        (ElementState::Pressed, Some(VirtualKeyCode::S)) => {
            camera.position = camera.position - camera.front * camera.speed;
        }

        (ElementState::Pressed, Some(VirtualKeyCode::A)) => {
            camera.position =
                camera.position - normalize(cross(camera.front, camera.up)) * camera.speed;
        }

        (ElementState::Pressed, Some(VirtualKeyCode::D)) => {
            camera.position =
                camera.position + normalize(cross(camera.front, camera.up)) * camera.speed;
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Add)) => {
            camera.fov += 1_f32.to_radians();
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Subtract)) => {
            camera.fov -= 1_f32.to_radians();
        }

        (ElementState::Released, Some(VirtualKeyCode::Tab)) => {
            camera.use_perspective = !camera.use_perspective;
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Up)) => {
            for vertex in &mut object.vertices {
                vertex.position[2] -= 0.1;
            }
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Down)) => {
            for vertex in &mut object.vertices {
                vertex.position[2] += 0.1;
            }
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Left)) => {
            for vertex in &mut object.vertices {
                vertex.position[0] -= 0.1;
            }
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Right)) => {
            for vertex in &mut object.vertices {
                vertex.position[0] += 0.1;
            }
        }

        (ElementState::Pressed, Some(VirtualKeyCode::Space)) => {
            for vertex in &mut object.vertices {
                vertex.position[1] += 0.1;
            }
        }

        (ElementState::Pressed, Some(VirtualKeyCode::LShift)) => {
            for vertex in &mut object.vertices {
                vertex.position[1] -= 0.1;
            }
        }

        _ => {}
    }
}

fn ortho_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    Mat4::new(
        Vec4::new(
            2. / (right - left),
            0.,
            0.,
            -(right + left) / (right - left),
        ),
        Vec4::new(
            0.,
            2. / (top - bottom),
            0.,
            -(top + bottom) / (top - bottom),
        ),
        Vec4::new(0., 0., -2. / (far - near), -(far + near) / (far - near)),
        Vec4::new(0., 0., 0., 1.),
    )
}
