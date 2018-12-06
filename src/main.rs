#[macro_use]
extern crate glium;

use glium::Surface;
use glium::glutin;

mod graphics;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let vertex_shader_src = String::new();
    let fragment_shader_src = String::new();

    let program = graphics::core::simple_program(
        &display,
        "shaders/vertex.vert",
        "shaders/fragment.frag"
    ).unwrap();

    let triangle_mesh = graphics::shapes::make_unit_cube(&display).unwrap();

    let proj_slice = nalgebra::Perspective3::new(16./10., 0.5, -1., 10.).to_homogeneous();
    let proj : &[[f32; 4] ;4] = proj_slice.as_ref();

    let uniforms = uniform! {
        camera_projection : *proj
    };

    let mut n : u32 = 0;
    events_loop.run_forever(|event|{

        match event {
            glutin::Event::WindowEvent {event, ..} => {
                match event {
                    glutin::WindowEvent::CloseRequested => {
                        println!("Got break request!");
                        return glium::glutin::ControlFlow::Break;
                    },
                    _ => ()
                }
                let mut target = display.draw();
                target.clear_color(1.,1.,1.,0.);
                triangle_mesh.draw(&mut target, &program,
                                   &uniforms,
                                   &Default::default()).unwrap();
                target.finish().unwrap();
                n += 1;
                println!("Hello world {}", n);
            },
            _ => ()
        }

        glium::glutin::ControlFlow::Continue
    })
}
