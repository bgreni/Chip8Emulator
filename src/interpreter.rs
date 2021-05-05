/// Virtual machine implementation


use rand::Rng;
use std::io::{BufWriter, Read};

/// Size of the RAM in bytes
const RAM_SIZE: usize = 4096;
/// Depth of the stack
const STACK_SIZE: usize = 256;
/// Number of data registers, i.e. `V0` .. `VF`
const NUM_DATA_REGISTERS: usize = 16;
/// Memory address for programm (ROM) start
const PROGRAM_START: usize = 0x200;
/// CPU clock speed
const CLOCK_HZ: f32 = 600.0;

/// Memory address of built-in font sprites
const FONT_ADDR: usize = 0;
/// Number of rows in one font sprite
const FONT_HEIGHT: usize = 5;
/// Size of one font sprite
const FONT_BYTES: usize = FONT_HEIGHT * 16;
/// Data of the built-in font
const FONT: [u8; FONT_BYTES] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
/// Width of the screen in pixels
const SCREEN_WIDTH: usize = 64;
/// Height of the screen in pixels
const SCREEN_HEIGHT: usize = 32;
/// Total number of pixels of the screen
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/// Number of keys on the keypad
const NUM_KEYS: usize = 16;


pub struct Interpreter {
    reg: [u8; NUM_DATA_REGISTERS],
    i: usize,
    pc: usize,
    sp: usize,
    stack: [usize; STACK_SIZE],
    ram: [u8; RAM_SIZE],

    timer: u8,
    t_tick: f32,

    sound_timer: u8,
    st_tick: f32,

    screen: [u8; SCREEN_PIXELS],
    keys: [u8; NUM_KEYS],
    waiting_on_key: Option<usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut inter = Interpreter {
            reg: [0; NUM_DATA_REGISTERS],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; STACK_SIZE],
            ram: [0; RAM_SIZE],

            timer: 0,
            t_tick: 0.0,

            sound_timer: 0,
            st_tick: 0.0,

            screen: [0; SCREEN_PIXELS],
            keys: [0; NUM_KEYS],
            waiting_on_key: None,
        };

        for i in 0..FONT_BYTES {
            inter.ram[FONT_ADDR + i] = FONT[i];
        }

        return inter;
    }

    pub fn load_program(&mut self, )
}
