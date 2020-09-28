
#![allow(warnings)]
extern crate piston_window;
use std::{thread, time, env};
use piston_window::*;
use piston_window::color::BLACK;
// use piston::input::Input;

mod hardware;
mod interpreter;
mod program_handler;
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
use crate::drivers::input::Inputs;
use crate::drivers::screen::Screen;
use crate::traits::chip_8_interpreter::Chip8Interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut inter: Interpreter = Interpreter::new();
    let mut input_driver = Inputs::new();
    let mut screen_driver = Screen::new();


    inter.load_program(filename);

    while let Some(event) = screen_driver.window.next() {

        // println!("{:?}", event.press_args());



        match event {
            Event::Input(inp,j) => {
                match inp {
                    Input::Button(but) => {
                        if but.state == ButtonState::Release{
                            continue
                        }
                        println!("{:?}", but);
                        let mut key = input_driver.check_key_presses(Some(but.button));
                        if key != None && inter.wait_for_key {
                            let pressed = key.unwrap();
                            println!("{}", pressed);
                            inter.set_reg(inter.key_reg, pressed);
                            inter.wait_for_key = false;
                        } else {
                            continue;
                        }
                    },
                    _=>{}
                }
            },
            _ => {
                if !inter.wait_for_key {
                    inter.fetch_next_instruction();
                    inter.run_op(&mut screen_driver, &mut input_driver, event.press_args());
                    screen_driver.update_screen(&event);
                }
            }
        }

        // std::thread::sleep_ms(10);
    }
}
