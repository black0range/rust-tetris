use crate::tetris::gamefield::GameField;
use crate::tetris::piece::Piece;
use crate::tetris::color::Color;
use crate::graphics::renderer::Renderer;
use rand::thread_rng;


pub struct TetrisManager {
    game_field: GameField,
    game_piece: Piece,
    rng: rand::rngs::ThreadRng,
    i: i32
}

impl TetrisManager {
    pub fn new(width: i32, height: i32) -> TetrisManager {
        let mut rng = rand::thread_rng();
        let mut piece = Piece::from_rng(&mut rng);
        piece.set_position((width / 2, 0));

        TetrisManager {
            game_field: GameField::new(width, height),
            game_piece: piece,
            rng: rng,
            i:0
        }
    }

    pub fn step(&mut self) -> bool {
        let down_one = self.game_piece.move_down();
        let new_coords = down_one.coordinates();
        let hits_floor = self.game_field.hits_floor(&new_coords);
        let hits_block = self.game_field.contains_any(new_coords.as_ref());
        if hits_floor || hits_block {
            self.game_field.insert_blocks(
                &self.game_piece.coordinates(),
                self.game_piece.color()
            );
            let mut rows: Vec<i32> = new_coords.iter().map(|(_,y)| *y)
                .collect();
            rows.sort_unstable();
            rows.dedup();
            for row in rows {
                if (self.game_field.is_row_full(row)){
                    self.game_field.delete_row(row);
                }
            };
            return false
        } else {
            self.game_piece = down_one;
            return true
        }
    }

    pub fn rotate_right(&mut self) {
        let rotated = self.game_piece.rotate_right();

        if self.game_field.valid_piece(rotated) {
            self.game_piece = rotated;
        }
    }

    pub fn rotate_left(&mut self) {
        let rotated = self.game_piece.rotate_left();
        if self.game_field.valid_piece(rotated) {
            self.game_piece = rotated.clone();
        }
    }

    pub fn move_left(&mut self) {
        let moved = self.game_piece.move_left();
        if self.game_field.valid_piece(moved) {
            self.game_piece = moved;
        }
    }

    pub fn move_right(&mut self) {
        let moved = self.game_piece.move_right();
        if self.game_field.valid_piece(moved) {
            self.game_piece = moved;
        }
    }

    pub fn tick(&mut self) {
        let stepped = self.step();
        if !stepped {
            self.game_piece = Piece::from_rng(&mut self.rng);
            self.game_piece.set_position((self.game_field.width() / 2, 0));

        }
    }

    // Returns al element given in a (0,0) - (1,1)  space
    pub fn elems(&self) -> Vec<((f32,f32), Color)> {
        let piece_coords = self.game_piece.coordinates();
        let game_piece = piece_coords
            .iter().map(|cord|(*cord, self.game_piece.color()));
        let mut res = self.game_field.get_blocks();
        res.extend(game_piece);
        res.iter().map(|((x,y), color)| {
            let f_x = *x as f32 * -1. / self.num_columns() as f32;
            let f_y = *y as f32 * -1. / self.num_rows() as f32;
            ((f_x, f_y), *color)
        }).collect()
    }

    pub fn num_columns(&self) -> i32 {
        return self.game_field.width()
    }

    pub fn num_rows(&self) -> i32 {
        return self.game_field.height()
    }
}
