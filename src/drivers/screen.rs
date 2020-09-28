use bit_vec::BitVec;
use piston_window::*;
use piston_window::color::{BLACK, WHITE};

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 320;
const PIXEL_SIZE: f64 = 10.0;

pub struct Screen {
    width: usize,
    height: usize,
    screen: Vec<BitVec>,
    pub window: PistonWindow,
    collision: u8
}

impl Screen {
    pub fn new() -> Self {
        return Screen {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            screen: get_screen(),
            window: WindowSettings::new("SOME SHIT", [640, 320]).exit_on_esc(true).build().unwrap(),
            collision: 0
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen.iter_mut().for_each(|mut line|
            line.iter().for_each(|mut cell| cell = false));
    }

    pub fn draw_sprite(&mut self, mut x: usize, mut y: usize, sprite: Vec<u8>) -> u8 {
        self.collision = 0;
        // check if x or y values of sprite need to be wrapped
        x = self.wrapx(x);
        y = self.wrapy(y);
        for byte in sprite {
            self.draw_byte(x, y, byte);
            y += 1;
        }

        return self.collision;
    }

    pub fn draw_byte(&mut self, x: usize, y: usize, byte: u8) {
        let b_vec = BitVec::from_bytes(&[byte]);
        for i in 0..b_vec.len() {
            if b_vec[i] {
                if self.screen[y][x + i] {
                    self.collision = 1;
                }
                self.screen[y].set(x + i, true);
            }
        }
    }

    pub fn update_screen(&mut self, event: &Event) {
        let s = &self.screen;
        self.window.draw_2d(event, |context, graphics, _device| {
            clear(WHITE, graphics);
            for line in 0..s.len() {
                for pixel in 0..s[line].capacity() {
                    if s[line][pixel] {
                        rectangle(BLACK,
                                  [pixel as f64*10.0, line as f64*10.0, PIXEL_SIZE, PIXEL_SIZE],
                                        context.transform,
                                    graphics);
                    }
                }
            }
        });
    }


    /// checks if x coordinate of a sprite will result in any part of it being drawn off
    /// of the map and wraps accordingly
    pub fn wrapx(&self, x: usize) -> usize {
        if x < 0 {
            return SCREEN_WIDTH - 8;
        } else if x + 8 > SCREEN_WIDTH  {
            return 0;
        }
        return x;
    }

    /// checks if y coordinate of a sprite will result in any part of it being drawn off
    /// of the map and wraps accordingly
    pub fn wrapy(&self, y: usize) -> usize {
        if (y < 0) {
            return SCREEN_WIDTH - 8;
        } else if (y + 8 > SCREEN_HEIGHT)  {
            return 0;
        }
        return y;
    }
}

pub fn get_screen() -> Vec<BitVec> {
    let mut s =  Vec::new();
    for i in 0..SCREEN_HEIGHT {
        s.push(BitVec::from_elem(SCREEN_WIDTH, false));
    }

    return s;
}