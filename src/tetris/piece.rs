use crate::tetris::color::Color;


// Named according to wikipedia, couldn't figure out good names for all of them
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    I = 0,
    J,
    L,
    O,
    S,
    T,
    Z,
}

const PIECE_TYPES : [PieceType;7] =
    [PieceType::I,PieceType::J,PieceType::L,
     PieceType::O,PieceType::S,PieceType::T,
     PieceType::Z];

impl PieceType {
    pub fn from_rng<R: rand::Rng>(rng: &mut R) -> PieceType {
        let n: usize = rng.gen();
        let t = PIECE_TYPES[n % PIECE_TYPES.len()];
        t
    }
}

const I_LAYOUT : [[(i32,i32);4];2] =
    [
        [(0,0),(1,0),(2,0),(3,0)],
        [(2,-1),(2,0),(2,1),(2,2)],
    ];

const J_LAYOUT : [[(i32,i32);4];4] =
    [
        [(-1,-1),(-1,0),(0,0),(1,0)],
        [(1,-1),(0,-1),(0,0),(0,1)],
        [(-1,0),(0,0),(1,0),(1,1)],
        [(-1,1),(0,-1),(0,0),(0,1)]
    ];

const L_LAYOUT : [[(i32,i32);4];4] =
    [
        [(1,-1),(-1,0),(0,0),(1,0)],
        [(1,1),(0,-1),(0,0),(0,1)],
        [(-1,0),(0,0),(1,0),(-1,1)],
        [(-1,-1),(0,-1),(0,0),(0,1)]
    ];

const O_LAYOUT : [[(i32,i32);4]; 1] =
    [[(0,0), (1,0), (0,1), (1,1)]];

const S_LAYOUT : [[(i32,i32);4];2] =
    [
        [(-1,0),(0,0),(0,-1),(1,-1)],
        [(0,-1),(0,0), (1,0),(1,1)],
    ];
const T_LAYOUT : [[(i32,i32);4];4] =
    [
        [(0,-1),(-1,0),(0,0),(1,0)],
        [(1,0),(0,-1),(0,0),(0,1)],
        [(-1,0),(0,0),(1,0),(0,1)],
        [(-1,0),(0,1),(0,0),(0,-1)]
    ];
const Z_LAYOUT : [[(i32,i32);4];2] =
    [
        [(-1,-1),(0,-1),(0,0),(1,0)],
        [(1,-1),(1,0),(0,0),(0,1)],
    ];

fn get_base_coordinates(
    piece_type: PieceType,
    rotation: Rotation
) -> [(i32,i32); 4] {
    let rot = rotation as usize;
    let res = match piece_type {
        PieceType::I => { I_LAYOUT.as_ref()},
        PieceType::J => { J_LAYOUT.as_ref()},
        PieceType::L => { L_LAYOUT.as_ref()},
        PieceType::O => { O_LAYOUT.as_ref()},
        PieceType::S => { S_LAYOUT.as_ref()},
        PieceType::T => { T_LAYOUT.as_ref()},
        PieceType::Z => { Z_LAYOUT.as_ref()},
    };

    res[rot % res.len()]
}


#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    Up = 0, Right , Down, Left
}

impl Default for Rotation {
    fn default() -> Rotation {
        Rotation::Up
    }
}

impl Rotation {
    pub fn rotate_left(self) -> Rotation {
        let n = ((((self as i8) - 1) % 4) + 4) % 4;
        println!("ASD {}", n);
        unsafe { std::mem::transmute(n)}
    }

    pub fn rotate_right(self) -> Rotation {
        let n = ((((self as i8) + 1) % 4) + 4) % 4;
        unsafe { std::mem::transmute(n)}
    }

}



#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    rotation: Rotation,
    position: (i32, i32)
}

impl Piece {

    pub fn from_rng<R: rand::Rng>(mut rng: &mut R) -> Piece {
        let p = PieceType::from_rng(&mut rng);
        let c = Color::from_rng(&mut rng);
        Piece::new(
            p,
            c,
            (0,0)
        )
    }

    pub fn new(
        piece_type: PieceType,
        color: Color,
        position: (i32, i32)
    ) -> Piece {
        Piece {
            color: color,
            piece_type: piece_type,
            rotation: Rotation::Up,
            position: position
        }
    }

    pub fn set_position(&mut self, coords: (i32, i32)) {
        self.position = coords
    }

    pub fn coordinates(&self) -> [(i32,i32);4]{
        let mut coords = get_base_coordinates(self.piece_type, self.rotation);
        let (pos_x, pos_y) = self.position;
        for i in 0 .. coords.len() {
            let (x,y) = coords[i];
            let translated = (x + pos_x, y + pos_y);
            coords[i] = translated
        };
        coords
    }

    pub fn rotate_right(self) -> Piece {
        Piece {
            rotation: self.rotation.rotate_right(),
            .. self
        }
    }

    pub fn rotate_left(self) -> Piece {
        Piece {
            rotation: self.rotation.rotate_left(),
            .. self
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn move_down(&self) -> Piece {
        let (x,y) = self.position;
        Piece{
            position: (x, y + 1),
            .. *self
        }
    }

    pub fn move_left(&self) -> Piece {
        let (x,y) = self.position;
        Piece{
            position: (x - 1, y),
            .. *self
        }
    }

    pub fn move_right(&self) -> Piece {
        let (x,y) = self.position;
        Piece{
            position: (x + 1, y),
            .. *self
        }
    }
}
