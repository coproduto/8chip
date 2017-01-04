#[macro_use]
extern crate gfx;
extern crate gfx_window_glfw;
extern crate glfw;

extern crate eightchip;

use glfw::{Action, Context, Key};

use gfx::traits::FactoryExt;
use gfx::Device;

use eightchip::chip8::Chip8;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [ 1.0, 0.0, 0.0 ] },
    Vertex { pos: [  0.5, -0.5 ], color: [ 0.0, 1.0, 0.0 ] },
    Vertex { pos: [  0.0,  0.5 ], color: [ 0.0, 0.0, 1.0 ] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .ok().expect("Failed to initialize GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "8CHIP", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.set_key_polling(true);
    window.make_current();
    
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);

    let (device, mut factory, color_view, depth_view) =
        gfx_window_glfw::init(&mut window);
    
    /*let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()
    ).unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color
    };*/
    
    let chip8 = Chip8::new();

    'main: loop {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => break 'main,
                _ => {},
            }
        }

        /*encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        //encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();*/
    }
}
