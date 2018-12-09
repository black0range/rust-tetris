use crate::tetris::color::Color;
use crate::tetris::piece::Piece;

pub struct GameField {
    width: i32,
    height: i32,
    game_field: Vec<Option<Color>>,
}

impl GameField {

    pub fn new(width: i32, height: i32) -> GameField {
        GameField {
            width: width,
            height: height,
            game_field: vec![Option::None; width as usize * height as usize],
        }
    }

    fn valid_index(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.width) && (0 <= y && y < self.height)
    }

    fn index_of(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_block(&mut self, x: i32, y: i32, color: Color) {
        let i = self.index_of(x,y);
        self.game_field[i] = Option::Some(color)
    }

    pub fn clear_block(&mut self, x: i32, y: i32) {
        let i = self.index_of(x,y);
        self.game_field[i] = Option::None;
    }


    fn from_index(&self, i: i32) -> (i32,i32) {
        use std::ops::Div;
        let y = i.div(self.width);
        let x = i - y * self.width;
        (x,y)
    }

    pub fn value_of(&self, x: i32, y: i32) -> Option<Color> {
        self.game_field[self.index_of(x,y)]
    }

    pub fn contains_node(&self, x: i32, y: i32) -> bool {
        self.valid_index(x,y) && self.value_of(x,y).is_some()
    }

    pub fn contains_any(&self, nodes : &[(i32,i32)]) -> bool {
        nodes.iter().any(|(x,y)| {self.contains_node(*x,*y)})
    }

    pub fn valid_piece(&self, piece: Piece) -> bool {
        let blocks = piece.coordinates();
        return !self.contains_any(&blocks)
            && !self.hits_wall(&blocks)
            && !self.hits_floor(&blocks)
    }

    pub fn hits_floor(&self, nodes : &[(i32,i32)]) -> bool {
        nodes.iter().any(|(_,y)| {*y >= self.height})
    }

    pub fn hits_wall(&self, nodes : &[(i32,i32)]) -> bool {
        nodes.iter().any(|(x,_)| {*x >= self.width || *x<0})
    }

    pub fn collisions(&self, nodes : &[(i32,i32)]) -> Vec<(i32,i32)> {
        let res : Vec<(i32,i32)>;
        res = nodes.iter()
            .filter(|(x,y)| self.contains_node(*x,*y))
            .map(|e|{*e}).collect();
        res
    }

    pub fn is_row_full(&self, row: i32) -> bool {
        for x in 0 .. self.width {
            if !self.contains_node(x, row) {
                return false
            }
        };
        true
    }

    pub fn get_blocks(&self) -> Vec<((i32,i32), Color)> {
        let mut res = Vec::new();
        for i in 0 .. self.game_field.len() {
            match self.game_field[i] {
                Option::None => (),
                Option::Some(color) => {
                    res.push((self.from_index(i as i32), color))
                }
            }
        };
        res
    }

    pub fn insert_blocks(&mut self, blocks: &[(i32,i32)], color: Color) {
        for (x,y) in blocks {
            if !self.contains_node(*x,*y) {
                self.set_block(*x,*y, color)
            }
        }
    }

    pub fn delete_row(&mut self, row: i32) {
        let iter_start = (self.width * (row + 1) - 1) as usize;
        let iter_end = self.width as usize;
        for i in (iter_end .. (iter_start + 1)).rev()  {
            self.game_field[i] = self.game_field[i - self.width as usize]
        }
        for x in 0 .. self.width as usize {
            self.game_field[x] = Option::None
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
