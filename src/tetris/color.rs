#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red = 0,
    Green,
    Blue,
    White,
    Magenta,
    Yellow
}

const COLORS : [Color; 6] =
    [Color::Red, Color::Green, Color::Blue,
     Color::White, Color::Magenta, Color::Yellow];

impl Into<(f32,f32,f32)> for &Color {
    fn into(self) -> (f32,f32,f32) {
        match self {
            Color::Red => (1.,0.,0.),
            Color::Green => (0.,1.,0.),
            Color::Blue => (0.,0.,1.),
            Color::White => (1.,1.,1.),
            Color::Magenta => (1.,0.,1.),
            Color::Yellow => (1.,1.,0.)
        }
    }
}

impl Color {
    pub fn from_rng<R: rand::Rng>(rng: &mut R) -> Color {
        let n: usize = rng.gen();
        COLORS[n % COLORS.len()]
    }
}
