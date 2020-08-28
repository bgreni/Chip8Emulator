use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::EventPump;
use crate::drivers::input::{Inputs, check_keys};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    width: usize,
    height: usize,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    window: Window
}

impl Screen {
    pub fn new(sdl_context: &Sdl) -> Self {
        return Screen {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            window: get_window(&sdl_context)
        }
    }

    pub fn show(&mut self) {
        self.window.show();
    }
}

fn get_window(sdl_context: &sdl2::Sdl) -> Window {
    let video = sdl_context.video().expect("failed to get sdl2 video context");
    let window = video.window("Keymod Test", 640, 480)
        .resizable()
        .opengl()
        .build()
        .expect("failed to create window");
    return window;
}