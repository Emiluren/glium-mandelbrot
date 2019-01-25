#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-1.0, -1.0] },
                Vertex { position: [-1.0,  1.0] },
                Vertex { position: [ 1.0, -1.0] },
                Vertex { position: [ 1.0,  1.0] },
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[
            0u16, 1, 2,
            1, 3, 2,
        ]
    ).unwrap();

    // compiling shaders and linking them together
    let program = program!(
        &display,
        330 => {
            vertex: include_str!("shader.vert"),
            fragment: include_str!("shader.frag"),
        },
    ).unwrap();

    let mut scale: f32 = 1.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;

    // the main loop
    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::WindowEvent::CloseRequested => return glutin::ControlFlow::Break,
                // Redraw the triangle when the window is resized.
                glutin::WindowEvent::MouseWheel { delta, .. } => {
                    if let glutin::MouseScrollDelta::LineDelta(_, scroll) = delta {
                        if scroll > 0.0 {
                            scale *= 1.1;
                        } else if scroll < 0.0 {
                            scale *= 0.9;
                        }
                    }
                },
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    let state = input.state == glutin::ElementState::Pressed;
                    match input.scancode {
                        0x11 => { up_pressed = state }, // W
                        0x1F => { down_pressed = state }, // S
                        0x1E => { left_pressed = state }, // A
                        0x20 => { right_pressed = state }, // D
                        _ => {}
                    }
                },
                _ => (),
            },
            _ => (),
        }

        let vx = if left_pressed && !right_pressed {
            -1.0
        } else if right_pressed && !left_pressed {
            1.0
        } else {
            0.0
        };
        let vy = if down_pressed && !up_pressed {
            -1.0
        } else if up_pressed && !down_pressed {
            1.0
        } else {
            0.0
        };

        x += vx * 0.1 * scale;
        y += vy * 0.1 * scale;

        // building the uniforms
        let uniforms = uniform! { scale: scale, offset: [x, y] };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));

        glutin::ControlFlow::Continue
    });
}
