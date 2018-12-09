extern crate glium;

use glium::Surface;
use crate::graphics::shapes;
use crate::tetris::manager::TetrisManager;

mod graphics;
mod tetris;

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

    // let triangle_mesh =renderer.load_mesh_with(
    //     &String::from("unit_triangle"),
    //     &|f| { shapes::make_unit_triangle(f) }
    // ).unwrap();

    let mut camera = graphics::camera::Camera::default();
    camera.aspect(3.1415);
    camera.set_far(10.);
    camera.set_near(1.);
    camera.position(0.,0.,-3.);

    let mut do_loop = true;


    let mut manager = TetrisManager::new(20, 20);
    let mut i = 0;
    let mut last_tick = std::time::Instant::now();
    while do_loop {
        events_loop.poll_events(|event|{
            match event {
                glium::glutin::Event::WindowEvent {event, ..} => {
                    use glium::glutin::WindowEvent;
                    use glium::glutin::VirtualKeyCode;
                    use glium::glutin::ElementState;
                    match event {
                        WindowEvent::Resized(size) => {
                            camera.aspect_of(size.into());
                        },
                        WindowEvent::KeyboardInput{input,..} => {
                            if input.state == ElementState::Pressed {
                                match input.virtual_keycode {
                                    Option::Some(code) => {
                                        match code {
                                            VirtualKeyCode::W => {
                                                manager.rotate_right();
                                            },
                                            VirtualKeyCode::A => {
                                                manager.move_left();
                                            },
                                            VirtualKeyCode::D => {
                                                manager.move_right();
                                            },
                                            VirtualKeyCode::Space => {
                                                manager.tick();
                                            },
                                            _ => ()
                                        }
                                    },
                                    Option::None => ()
                                }
                            }
                        },
                        WindowEvent::CloseRequested => {
                            println!("Got break request!");
                            do_loop = false;
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        });


        if last_tick.elapsed().as_secs() > 1 {
            manager.tick();
            last_tick = std::time::Instant::now()
        }

        let scale = 0.5 / manager.num_columns() as f32;
        let nodes = manager.elems();
        let mut elems = nodes.iter().map(|((x,y), color)| {
            let mut obj = graphics::renderer::RenderObject::new(cube_mesh);
            let (r,g,b) = color.into();
            obj.trans().move_to(*x, *y, 0.);
            obj.trans().add_position(0.5, 0.5, 0.);
            obj.trans().set_uniform_scale(scale);
            obj.rgb(r,g,b);
            obj
        }).collect();

        renderer.render(&mut elems, &mut camera);
    }
}
