use sdl2;

mod hardware;
mod interpreter;
mod program_loader;
mod traits {
    pub mod instructions;
    pub mod chip_8_interpreter;
}
mod drivers {
    pub mod input;
    pub mod screen;
}

// use std::env;
use crate::interpreter::Interpreter;
// use crate::traits::Chip8Interpreter::Chip8Interpreter;
use crate::traits::instructions::{Instructions, Helpers};
use crate::drivers::input::{Inputs, check_keys};
use crate::drivers::screen::Screen;
use std::{thread, time};
use sdl2::pixels::Color;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    let sdl_context = sdl2::init().unwrap();
    
    let mut inter: Interpreter = Interpreter::new();
    let mut input_driver = Inputs::new(&sdl_context);
    let mut screen_driver = Screen::new(&sdl_context);
    'main: loop {

        inter.set_reg(1, 1);
        inter.sknp(1, &input_driver);

        // inter.ldk(1);

        let should_quit = input_driver.check_quit();
        if should_quit {
            break 'main;
        }
        if inter.wait_for_key {
            let mut keys = input_driver.check_key_presses();
            let mut key_pressed = check_keys(&keys);
            if key_pressed != None {
                let pressed = key_pressed.unwrap();
                println!("{}", pressed);
                inter.set_reg(1, pressed);
                inter.wait_for_key = false;
            }
        }
        std::thread::sleep_ms(100);
    }
}
