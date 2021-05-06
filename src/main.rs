
#![allow(warnings)]

extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate sdl2_window;

use opengl_graphics::{ GlGraphics, OpenGL };
use piston_window::{
    UpdateEvent,
    RenderEvent,
    PressEvent,
    ReleaseEvent,
    AdvancedWindow,
    PistonWindow,
    WindowSettings
};
use piston::input::{ Button, Key };
use sdl2_window::Sdl2Window;

mod hardware;
mod interpreter;
mod program_handler;
mod traits {
    pub mod instructions;
}
mod drivers {
    pub mod input;
}

use crate::interpreter::Interpreter;
use crate::traits::instructions::{Instructions, Helpers};
use crate::drivers::input::{keymap};
use std::env;
use graphics::*;

static TITLE: &str = "GAME";
static BEEP_TITLE: &str = "BEEP";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut inter: Interpreter = Interpreter::new();
    inter.load_program(filename);

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(
        TITLE,
        [800, 400]
    )
        .exit_on_esc(true)
        // .opengl(opengl)
        .build()
        .unwrap();
    let ref mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        if let Some(args) = e.update_args() {
            inter.step(args.dt as f32);

            if inter.is_beeping() {
                window.set_title(BEEP_TITLE.to_string());
            } else {
                window.set_title(TITLE.to_string());
            }
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                let r = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
                let off = [0.0, 0.0, 0.0, 1.0];
                let on = [1.0, 1.0, 1.0, 1.0];

                let w = args.window_size[0] / 64.0;
                let h = args.window_size[1] / 32.0;
                // println!("{:?}", inter.screen);
                for (y,row) in inter.screen_rows().enumerate() {
                    for (x,byte) in row.iter().enumerate() {
                        let x = x as f64 * w;
                        let y = y as f64 * h;
                        let color = match *byte { 0 => off, _ => on };
                        r.color(color).draw([x, y, w, h], &c.draw_state, c.transform, gl);
                    }
                }
            });
        }

        if let Some(key) = keymap(e.press_args()) {
            inter.set_key(key);
        }
        if let Some(key) = keymap(e.release_args()) {
            inter.unset_key(key);
        }
    }
}
