pub struct Camera {
    uniform_matrix: nalgebra::Matrix4<f32>,

    proj_cached: nalgebra::Matrix4<f32>,
    trans_cached: nalgebra::Matrix4<f32>,
    look_at_cached: nalgebra::Matrix4<f32>,


    changed_proj: bool,
    changed_trans: bool,
    changed_look_at: bool,

    aspect: f32,
    fovy: f32,
    near: f32,
    far: f32,

    position: nalgebra::Vector3<f32>,
    up: nalgebra::Vector3<f32>,
    at: nalgebra::Vector3<f32>,
}

impl Default for Camera {
    fn default() -> Camera {

        Camera  {
            uniform_matrix: nalgebra::Matrix4::identity(),
            proj_cached: nalgebra::Matrix4::identity(),
            trans_cached: nalgebra::Matrix4::identity(),
            look_at_cached: nalgebra::Matrix4::identity(),

            changed_proj: true,
            changed_trans: true,
            changed_look_at: true,

            aspect: 0.5, // Sqruare, why not?
            fovy: 0.78,  // 45 degrees
            near: 10.0,  // Completely arbitary
            far: 100.,  // Completely arbitary

            position: nalgebra::Vector3::new(0.,0.,0.),
            up: nalgebra::Vector3::new(0.,1.,0.),
            at: nalgebra::Vector3::new(0.,0.,1.),
        }
    }
}


impl Camera {

    pub fn as_primitive(&mut self) -> [[f32; 4]; 4] {
        if self.anything_changed() {
            self.update_matrix();
        };
        *self.uniform_matrix.as_ref()
    }

    fn  anything_changed(&self) -> bool {
        self.changed_proj || self.changed_trans || self.changed_look_at
    }

    fn set_changed_proj(&mut self) {
        self.changed_proj = true;
    }

    fn set_changed_trans(&mut self) {
        self.changed_trans = true;
    }

    fn set_changed_look_at(&mut self) {
        self.changed_look_at = true;
    }

    fn update_matrix(&mut self) -> &Camera{
        if self.changed_proj {
            self.proj_cached = nalgebra::Perspective3::new(
                self.aspect, self.fovy, self.near, self.far
            ).to_homogeneous();
        }

        if self.changed_trans {
            self.trans_cached = nalgebra::geometry::Translation::from(self.position)
                .to_homogeneous();
        }
        if self.changed_look_at {
            self.look_at_cached = nalgebra::Rotation3::look_at_rh(&self.at, &self.up)
                .to_homogeneous();
        }

        self.uniform_matrix = self.proj_cached * self.trans_cached * self.look_at_cached;
        self
    }

    pub fn aspect(&mut self, aspect: f32) -> &Camera{
        self.aspect = aspect;
        self.set_changed_proj();
        self
    }


    pub fn aspect_of(&mut self, (x,y): (f64,f64)) -> &Camera {
        self.aspect((y/x) as f32)
    }

    pub fn fovy(&mut self, fovy: f32) {
        self.fovy = fovy;
        self.set_changed_proj();
    }
    pub fn set_near(&mut self, near: f32)  -> &Camera {
        self.near = near;
        self.set_changed_proj();
        self
    }
    pub fn set_far(&mut self, far: f32) -> &Camera {
        self.far = far;
        self.set_changed_proj();
        self
    }

    pub fn horizontal_rotate(&mut self, angle: f32) -> &Camera {
        let rot_vec = self.at.cross(&self.up);
        self.at = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Unit::new_normalize(rot_vec),
            angle) * self.at;
        self.set_changed_look_at();
        self
    }

    pub fn vertical_rotate(&mut self, angle: f32) -> &Camera {
        let rot_vec = nalgebra::Vector3::new(0., 1., 0.);
        self.at = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Unit::new_normalize(rot_vec),
            angle) * self.at;
        self.set_changed_look_at();
        self
    }

    pub fn position(&mut self, x: f32, y: f32, z: f32) -> &Camera {
        self.move_to(x,y,z)
    }
    pub fn move_to(&mut self, x: f32, y: f32, z: f32) -> &Camera {
        self.position = nalgebra::Vector3::new(x,y,z);
        self.set_changed_trans();
        self
    }

    pub fn position_ve(&mut self, v: nalgebra::Vector3<f32>) -> &Camera {
        self.move_to_vec(v)
    }

    pub fn move_to_vec(&mut self, v: nalgebra::Vector3<f32>) -> &Camera {
        self.position = v;
        self.set_changed_trans();
        self
    }

    pub fn add_position(&mut self, v: &nalgebra::Vector3<f32>) -> &Camera{
        self.position += v;
        self.set_changed_trans();
        self
    }

    pub fn move_forwards(&mut self, d: f32) -> &Camera {
        let a = d * self.at;
        self.add_position(&a);
        self
    }

    pub fn move_backwards(&mut self, d: f32) -> &Camera {
        self.move_forwards(-d);
        self
    }

    pub fn move_sideways(&mut self, d: f32) -> &Camera {
        let a = self.at.cross(&self.up) * d;
        self.add_position(&a);
        self
    }

    pub fn move_up(&mut self, _d: f32) -> &Camera {
        let a = self.at.cross(&self.up);
        self.add_position(&a);
        self
    }

    pub fn up(&mut self, up: nalgebra::Vector3<f32>) -> &Camera {
        self.up = up;
        self.set_changed_look_at();
        self
    }

    pub fn set_look_at(&mut self, at: nalgebra::Vector3<f32>) -> &Camera {
        self.at = at;
        self.set_changed_look_at();
        self
    }
}
