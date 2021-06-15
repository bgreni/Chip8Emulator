
#![allow(warnings)]

extern crate sdl2;
extern crate bit_vec;
extern crate clap;
extern crate rand;

mod hardware;
mod interface;

use clap::{Arg, App};
use std::time::Duration;

use crate::hardware::chip8;
use crate::interface::{input, display};

static TITLE: &str = "GAME";
static BEEP_TITLE: &str = "BEEP";

const UI_SCALE: u32 = 8;
const WIDTH: u32 = chip8::WIDTH * UI_SCALE;
const HEIGHT: u32 = chip8::HEIGHT * UI_SCALE;

fn main() {

    let matches = App::new("Chip8 Interpreter")
        .version(chip8::version())
        .author("Brian Grenier <grenierb96@gmail.com")
        .about("Emulates Chip8 programs")
        .arg(Arg::with_name("ROM")
            .help("Path name of the ROM to run")
            .required(true))
        .get_matches();

    let mut inter = chip8::Chip8::default();
    inter.load_program(matches.value_of("ROM").unwrap());

    let sdl_context = sdl2::init().unwrap();
    let mut input = input::Input::new(&sdl_context);

    let mut window = display::Display::new(&sdl_context,
                    "Chip8 Emulator",
                    WIDTH,
                    HEIGHT);

    'main: loop {
        match input.poll(&mut inter.keys) {
            input::Command::Quit => break 'main,
            input::Command::Continue => {},
        }

        inter.run_cycle();

        if inter.draw {
            inter.draw = false;
            window.draw_frame(&inter.screen);
        }
    }

}
