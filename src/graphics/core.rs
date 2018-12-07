use glium::backend::{Facade};
use std::path::Path;
use glium::program::Program;
use std::fs::File;

use::std::error::Error;

#[derive(Debug)]
pub enum BufferCreationError {
    IndexError(glium::index::BufferCreationError),
    VertexError(glium::vertex::BufferCreationError)
}

impl std::error::Error for BufferCreationError {
    fn description(&self) -> &str {
        match self {
            BufferCreationError::IndexError(e) => e.description(),
            BufferCreationError::VertexError(e) => e.description()
        }
    }
}

impl std::fmt::Display for BufferCreationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {

        write!(fmt, "{}", self.description())
    }
}


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position : [f32; 3]
}

impl Vertex {
    pub const fn new_2d(x: f32, y: f32) -> Vertex {
        Vertex{position:[x, y, 0.]}
    }
    pub const fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex{position:[x, y, z]}
    }
}


glium::implement_vertex!(Vertex, position);


pub struct Mesh {
    index_buffer:  glium::IndexBuffer<u16>,
    vertex_buffer: glium::VertexBuffer<Vertex>
}


impl Mesh {

    pub fn draw<S, U>(
        &self,
        surface: &mut S,
        program: &glium::Program,
        uniforms: &U,
        params: &glium::DrawParameters
    ) -> Result<(), glium::DrawError>
    where S: glium::Surface, U: glium::uniforms::Uniforms  {
        surface.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            program,
            uniforms,
            params
        )
    }

    pub fn new<F> (
        facade: &F,
        indices: &[u16],
        vertices: &[Vertex]
    ) -> std::result::Result<Mesh, BufferCreationError>
    where F: Facade {
        glium::VertexBuffer::new(facade, vertices)
            .or_else(|e|{Result::Err(BufferCreationError::VertexError(e))})
            .and_then(|vb|{
                glium::index::IndexBuffer::new(
                    facade,
                    glium::index::PrimitiveType::TrianglesList,
                    indices)
                    .map_err(BufferCreationError::IndexError)
                    .map(|ib|{
                        Mesh{index_buffer: ib, vertex_buffer: vb}
                })
        })
    }

    pub fn new_triangles<F> (
        facade: &F,
        vertices: &[Vertex]
    ) -> Result<Mesh, BufferCreationError>
    where F: Facade {
        let indices : Vec<u16> = (0 .. vertices.len() as u16).collect();
        Mesh::new(facade, &indices, vertices)
    }
}




#[derive(Debug)]
pub enum ProgramCreationError {
    IOError(std::io::Error),
    ShaderError(glium::ProgramCreationError)
}

impl std::error::Error for ProgramCreationError {
    fn description(&self) -> &str {
        match self {
            ProgramCreationError::IOError(e) => e.description(),
            ProgramCreationError::ShaderError(e) => e.description()
        }
    }
}

impl std::fmt::Display for ProgramCreationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {

        write!(fmt, "{}", self.description())
    }
}


pub fn simple_program<P,F>(
    facade: &F,
    vertex_file_path: P,
    fragment_file_path: P
) -> Result<Program, ProgramCreationError>
where P: AsRef<Path>, F: Facade {
    let mut vertex_src = String::new();
    let mut fragment_src = String::new();
    use std::io::Read;
    {
        let vertex_file = File::open(vertex_file_path)
            .map_err(ProgramCreationError::IOError)?;
        let mut vertex_file_buffer = std::io::BufReader::new(vertex_file);
        vertex_file_buffer.read_to_string(&mut vertex_src)
            .map_err(ProgramCreationError::IOError)?;
    };
    {
        let fragment_file = File::open(fragment_file_path)
            .map_err(ProgramCreationError::IOError)?;
        let mut vertex_file_buffer = std::io::BufReader::new(fragment_file);
        vertex_file_buffer.read_to_string(&mut fragment_src)
            .map_err(ProgramCreationError::IOError)?;
    };

    glium::Program::from_source(facade, vertex_src.as_str(), fragment_src.as_str(), None).map_err(ProgramCreationError::ShaderError)
}
