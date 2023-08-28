use line_drawing::Bresenham;

pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = 600;
pub const PI: f32 = 3.1415;

#[derive(Debug, Clone)]
pub enum Color {
    rgb(String),
}

#[derive(Clone, Copy)]
pub enum SimpleColor {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}

pub struct Renderer {
    pub buffer: Vec<u32>,
}
impl Renderer {
    pub fn draw_pixel(&mut self, position: (u32, u32), color: SimpleColor) {
        if position.0 < WIDTH as u32
            && position.0 > 0
            && position.1 < HEIGHT as u32
            && position.1 > 0
        {
            self.buffer[(position.0 + position.1 * WIDTH as u32) as usize] = color as _;
        }
    }

    pub fn clear(&mut self, color: SimpleColor) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}

pub trait Draw {
    fn draw(&self, renderer: &mut Renderer, color: SimpleColor) {}
}

pub struct Rectangle {
    pub lenght: u32,
    pub height: u32,
    pub position: (u32, u32),
}
impl Rectangle {
    pub fn new(lenght: u32, height: u32, position: (u32, u32)) -> Self {
        Rectangle {
            lenght,
            height,
            position,
        }
    }
}
impl Draw for Rectangle {
    fn draw(&self, renderer: &mut Renderer, color: SimpleColor) {
        let pos_y = self.position.1;
        let pos_x = self.position.0;

        for y in pos_y..self.height + pos_y {
            for x in pos_x..self.lenght + pos_x {
                renderer.buffer[(y * WIDTH as u32 + x) as usize] = color as _;
            }
        }
    }
}

pub struct Line {
    pub pos_1: (u32, u32),
    pub pos_2: (u32, u32),
}
impl Line {
    pub fn new(pos_1: (u32, u32), pos_2: (u32, u32)) -> Line {
        Line { pos_1, pos_2 }
    }

    pub fn length_of_line(&self) -> f32 {
        let dx: f32 = (self.pos_1.0 as f32 - self.pos_2.0 as f32).abs();
        let dy: f32 = (self.pos_1.1 as f32 - self.pos_2.1 as f32).abs();
        return (dx * dx + dy * dy).powf(0.5);
    }
}
impl Draw for Line {
    fn draw(&self, renderer: &mut Renderer, color: SimpleColor) {
        for (x, y) in Bresenham::new(
            (self.pos_1.0 as i32, self.pos_1.1 as i32),
            (self.pos_2.0 as i32, self.pos_2.1 as i32),
        ) {
            renderer.draw_pixel((x as u32, y as u32), color);
        }
    }
}
