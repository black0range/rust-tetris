use nalgebra::{Vector4, Vector3, Matrix4};
use glium::backend::{Facade};
use crate::graphics::core::*;
use crate::graphics::camera::Camera;
use std::path::Path;
use glium::Surface;
use glium::uniform;
#[macro_use]

pub struct ModelTrans {
    uniform_matrix: Matrix4<f32>,
    changed: bool,
    position: Vector3<f32>,
    scale: Vector3<f32>,
    at: Vector3<f32>,
    up: Vector3<f32>
}

impl Default for ModelTrans {
    fn default() -> ModelTrans {
        ModelTrans {
            uniform_matrix: Matrix4::identity(),
            changed: true,
            position: Vector3::new(0.,0.,0.),
            scale: Vector3::new(1.,1.,1.),
            at: Vector3::new(0.,0.,1.),
            up: Vector3::new(0.,1.,0.),
        }
    }
}

impl ModelTrans {

    pub fn as_array(&mut self) -> [[f32; 4]; 4] {
        if self.changed {
            self.update_matrix();
        };
        *self.uniform_matrix.as_ref()
    }

    pub fn as_ref(&mut self) -> &[[f32; 4]; 4] {
        if self.changed {
            self.update_matrix();
        };
        self.uniform_matrix.as_ref()
    }

    fn update_matrix(&mut self) {
        let trans = nalgebra::geometry::Translation::from(self.position)
            .to_homogeneous();
        let mut scale = nalgebra::Matrix4::from_diagonal(
            &nalgebra::Vector4::new(
                self.scale[0],
                self.scale[1],
                self.scale[2],
                1.0
            )
        );

        let look_at = nalgebra::Rotation3::look_at_rh(&self.at, &self.up)
            .to_homogeneous();
        self.uniform_matrix = trans * scale
    }


    pub fn move_to(&mut self, x: f32, y: f32, z: f32) {
        self.position = nalgebra::Vector3::new(x,y,z);
        self.changed = true;
    }

    pub fn add_position(&mut self, x: f32, y: f32, z: f32) {
        self.position += Vector3::new(x,y,z);
        self.changed = true;
    }

    pub fn add_position_vec(&mut self, dir: &Vector3<f32>) {
        self.position += dir;
        self.changed = true;
    }

    pub fn set_up(&mut self, up: nalgebra::Vector3<f32>) {
        self.up = up;
        self.changed = true
    }

    pub fn set_look_at(&mut self, at: nalgebra::Vector3<f32>) {
        self.at = at;
        self.changed = true
    }

    pub fn set_uniform_scale(&mut self, v: f32) {
        self.scale = Vector3::new(v,v,v);
        self.changed = true
    }

    pub fn set_scale(&mut self, x: f32, y:f32, z:f32) {
        self.scale = Vector3::new(x,y,z);
        self.changed = true
    }

    pub fn set_scale_vec(&mut self, vec: Vector3<f32>) {
        self.scale = vec;
        self.changed = true
    }

    pub fn scale_by(&mut self, x: f32, y:f32, z:f32) {
        self.scale.component_mul(&Vector3::new(x,y,z));
        self.changed = true
    }

    pub fn scale_by_vec(&mut self, vec: Vector3<f32>) {
        self.scale.component_mul(&vec);
        self.changed = true
    }

    pub fn horizontal_rotate(&mut self, angle: f32) {
        let rot_vec = self.at.cross(&self.up);
        self.at = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Unit::new_normalize(rot_vec),
            angle) * self.at;
        self.changed = true;
    }

    pub fn vertical_rotate(&mut self, angle: f32) {
        let rot_vec = nalgebra::Vector3::new(0., 1., 0.);
        self.at = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Unit::new_normalize(rot_vec),
            angle) * self.at;
        self.changed = true;
    }

}


pub struct RenderObject {
    base_rgba:  Vector4<f32>,
    model_trans: ModelTrans,
    mesh_ref:    MeshRef,
}

impl RenderObject {
    pub fn new(mesh_ref: MeshRef) -> RenderObject {
        RenderObject {
            base_rgba: Vector4::new(1.,0.,0.,1.),
            model_trans: Default::default(),
            mesh_ref: mesh_ref
        }
    }
    pub fn rgb(&mut self, r: f32, g: f32, b: f32) {
        self.base_rgba = Vector4::new(r,g,b,self.base_rgba[3])
    }

    pub fn rgb_vec(&mut self, vec: Vector3<f32>) {
        self.base_rgba = Vector4::new(vec[0],vec[1],vec[2],self.base_rgba[3])
    }


    pub fn opacity(&mut self, v: f32) {
       self.base_rgba[3] = v
    }

    pub fn trans(&mut self) -> &mut ModelTrans {
        &mut self.model_trans
    }
}


// Dummy reference wrapper for mesh ids, wrapped so that we do not mistake it
// for normal integers.
#[derive(Hash, Debug, PartialEq, Eq, Copy)]
pub struct MeshRef {
    id : u64
}

impl Clone for MeshRef {
    fn clone(&self) -> MeshRef {
        *self
    }
}

struct MeshStore {
    mesh_ref: MeshRef,
    map: std::collections::HashMap<MeshRef, Mesh>,
    name_map: std::collections::HashMap<String, MeshRef>
}

impl MeshStore {
    pub fn new() -> MeshStore {
        MeshStore{
            mesh_ref: MeshRef{id:0},
            map: Default::default(),
            name_map: Default::default()
        }
    }
    // Hiding insert becaue it probaby should not be done outside the context of
    // the renderer
    fn insert(
        &mut self,
        name: &String,
        make: &Fn() -> Result<Mesh, BufferCreationError>,
    ) -> Result<MeshRef, BufferCreationError>
    {
        let content = self.name_map.get(name).map(|e| e.clone());
        match content {
            Option::Some(reference) => {
                return Result::Ok(reference);
            },
            Option::None => {
                let id = self.gen_id();
                let mesh = make()?;
                self.map.entry(id).or_insert(mesh);
                return Result::Ok(id)
            }
        }
    }

    pub fn get_mesh(&self, reference: &MeshRef) -> Option<&Mesh> {
        self.map.get(reference)
    }

    pub fn get_ref(&self, name: &String) -> Option<MeshRef> {
        self.name_map.get(name).map(|e|{*e})
    }

    fn gen_id(&mut self) -> MeshRef {
        let result = self.mesh_ref;
        self.mesh_ref.id += 1;
        return result
    }
}


pub struct Renderer<'a> {
    display: &'a glium::Display,
    program: Option<glium::Program>,
    mesh_store: MeshStore,
}


impl<'a> Renderer<'a> {

    pub fn new(
        display: &'a glium::Display
    ) -> Renderer<'a> {
        Renderer{
            display: display,
            program: Option::None,
            mesh_store: MeshStore::new(),
        }
    }

    pub fn mesh_store(&self) -> &MeshStore {
        &self.mesh_store
    }

    pub fn render(
        &mut self,
        objects :&mut Vec<RenderObject>,
        camera: &mut Camera
    ) {
        let mut target = self.display.draw();
        target.clear_color(0.,0.,0.,0.);
        let cam_mat = camera.as_primitive();
        let program = self.program.as_mut().unwrap();

        for obj in objects.into_iter() {
            let mesh_ref = obj.mesh_ref.clone();
            let model_mat = obj.model_trans.as_array();
            let uniforms = uniform! {
                camera_mat: cam_mat,
                model_mat: model_mat,
                rgba_color: *obj.base_rgba.as_ref()
            };

            let mesh = self.mesh_store.get_mesh(&mesh_ref)
                .unwrap();

            mesh.draw(
                &mut target,
                &program,
                &uniforms,
                &Default::default()
            ).unwrap()
        };

        target.finish();
    }

    pub fn load_mesh(
        &mut self,
        name: &String,
        indices: &[u16],
        vertices: &[Vertex]
    ) -> Result<&Renderer, BufferCreationError> {
        let f = self.display;
        self.mesh_store.insert(name, &|| {
            Mesh::new(f,indices, vertices)}
        )?;
        Result::Ok(self)
    }

    pub fn load_mesh_with(
        &mut self,
        name: &String,
        init_fn: &Fn(&glium::Display) -> Result<Mesh, BufferCreationError>
    ) -> Result<MeshRef, BufferCreationError>
    {
        let f = self.display;
        self.mesh_store.insert(name, &|| {
            init_fn(f)
        })

    }


    pub fn get_mesh(&self, name: &String) -> Option<MeshRef> {
        self.mesh_store.get_ref(name)
    }

    pub fn use_program<P>(
        &mut self,
        vertex_fp: P,
        fragment_fp: P
    ) -> Result<&Renderer, ProgramCreationError>
    where P: AsRef<Path> {
        self.program = crate::graphics::core::simple_program(
            self.display,
            vertex_fp,
            fragment_fp
        ).map(Option::Some)?;
        Result::Ok(self)
    }
}
