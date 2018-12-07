use crate::graphics::core::{Vertex, Mesh, BufferCreationError};
use glium::backend::{Facade};

const UNIT_TRIANGLE_VERTICES : [Vertex;3] = [
    Vertex::new_2d(0.,1.),
    Vertex::new_2d(1.,-1.),
    Vertex::new_2d(-1.,-1.)
];


const UNIT_TRIANGLE_INDICES : [u16;3] = [
    0,1,2
];


const UNIT_CUBE_VERTICES : [Vertex;8] = [
    Vertex::new(-1., 1., 1.),
    Vertex::new(1., 1., 1.),
    Vertex::new(1., 1.,-1.),
    Vertex::new(-1., 1.,-1.),
    Vertex::new(-1.,-1., 1.),
    Vertex::new(1.,-1., 1.),
    Vertex::new(1.,-1.,-1.),
    Vertex::new(-1.,-1.,-1.)
];

const UNIT_CUBE_INDICES : [u16;36] =  [
    7, 3, 2,
    7, 2, 6,
    6, 2, 1,
    6, 1, 5,
    5, 1, 0,
    5, 0, 4,
    4, 0, 3,
    4, 3, 7,
    3, 0, 1,
    3, 1, 2,
    7, 5, 4,
    7, 6, 5
];





pub fn make_unit_cube<F>(facade: &F) -> Result<Mesh, BufferCreationError>
where F: Facade {
    Mesh::new(facade, &UNIT_CUBE_INDICES, &UNIT_CUBE_VERTICES)
}


pub fn make_unit_triangle<F>(facade: &F) -> Result<Mesh, BufferCreationError>
where F: Facade {
    Mesh::new(facade, &UNIT_TRIANGLE_INDICES, &UNIT_TRIANGLE_VERTICES)
}
