extern crate glium;

use glium::Surface;
use crate::graphics::shapes;
mod graphics;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut renderer = graphics::renderer::Renderer::new(&display);
    renderer.use_program("shaders/vertex.vert", "shaders/fragment.frag")
        .unwrap();

    let cube_mesh = renderer.load_mesh_with(
        &String::from("unit_cube"),
        &shapes::make_unit_cube
    ).unwrap();

    let triangle_mesh =renderer.load_mesh_with(
        &String::from("unit_triangle"),
        &shapes::make_unit_cube
    ).unwrap();


    let mut cube = graphics::renderer::RenderObject::new(cube_mesh);

    let mut camera = graphics::camera::Camera::default()
        .aspect(0.5)
        .position(0.,0.,-10.);

    let mut do_loop = true;
    while do_loop {
        events_loop.poll_events(|event|{

            match event {
                glutin::Event::WindowEvent {event, ..} => {
                    use glutin::WindowEvent::*;
                    match event {
                        Resized(size) => {
                            camera.aspect_of(size.into());
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

        renderer.render([cube].into(), &mut camera);

        // triangle_mesh.draw(&mut target, &program,
        //                    &uniforms,
        //                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
