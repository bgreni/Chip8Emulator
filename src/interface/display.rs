use sdl2;
use sdl2::{pixels, rect::Rect, render::Canvas, video::Window};
use crate::hardware::chip8::PIXEL_COUNT;
use crate::UI_SCALE;

pub struct Display {
    canvas: Canvas<Window>
}

impl Display {
    pub fn new(sdl_ctx: &sdl2::Sdl, title: &str, width: u32, height: u32,) -> Self {
        let video = sdl_ctx.video().unwrap();
        let window = video
            .window(
                "chip8-rs",
                width,
                height
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        return Display { canvas };
    }

    pub fn draw_frame(&mut self, bitmap: &[u8; PIXEL_COUNT]) {
        for y in 0..32 {
            for x in 0..64 {
                self.canvas.set_draw_color(
                  match bitmap[self.point_to_index(x, y)] {
                      0 => pixels::Color::BLACK,
                      _ => pixels::Color::WHITE
                  }
                );
                let _ = self.canvas.fill_rect(Rect::new(
                    (x * UI_SCALE as usize) as i32,
                    (y * UI_SCALE as usize) as i32,
                    UI_SCALE as u32,
                    UI_SCALE as u32
                ));
            }
        }
        self.canvas.present();
    }

    fn point_to_index(&self, x: usize, y: usize) -> usize {
        return y * 64 + x;
    }
}