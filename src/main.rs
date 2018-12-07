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
    let mut camera = graphics::camera::Camera::default();
    camera.set_aspect(0.5);
    camera.move_to_dims(0.,0.,-10.);

    let mut do_loop = true;
    while do_loop {
        events_loop.poll_events(|event|{

            match event {
                glutin::Event::WindowEvent {event, ..} => {

                    use glutin::WindowEvent::*;
                    match event {
                        Resized(size) => {
                            camera.use_aspect_of(size.into())
                        },
                        KeyboardInput{input, ..} => {
                            match input.scancode {
                                17 => {
                                    // Wordward
                                    camera.move_forwards(0.5);
                                },
                                31 => {
                                    //Backwards
                                    camera.move_backwards(0.5);
                                },
                                _ => ()
                            }
                            //                        println!("Scancode: {}", input.scancode)
                        },
                        CloseRequested => {
                            println!("Got break request!");
                            do_loop = false;
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        });

        let mut target = display.draw();
        target.clear_color(1.,1.,1.,0.);

        let uniforms = uniform! {
            camera_projection : *camera.as_primitive(),
        };

        triangle_mesh.draw(&mut target, &program,
                           &uniforms,
                           &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
