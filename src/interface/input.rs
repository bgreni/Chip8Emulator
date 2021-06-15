use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::keyboard::{Keycode, KeyboardState, Scancode};


pub struct Input {
    event_pump: EventPump,
}

pub enum Command {
    Continue,
    Quit,
}

impl Input {
    pub fn new(context: &Sdl) -> Self {
        return Input {
            event_pump: context.event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self, keys: &mut [u8; 16]) -> Command {

        for event in self.event_pump.poll_iter() {
            if let Event::Quit {..} = event {
                return Command::Quit;
            }
        }

        let kb = KeyboardState::new(&self.event_pump);
        keys[0x0] = kb.is_scancode_pressed(Scancode::Num0) as u8;
        keys[0x1] = kb.is_scancode_pressed(Scancode::Num1) as u8;
        keys[0x2] = kb.is_scancode_pressed(Scancode::Num2) as u8;
        keys[0x3] = kb.is_scancode_pressed(Scancode::Num3) as u8;
        keys[0x4] = kb.is_scancode_pressed(Scancode::Num4) as u8;
        keys[0x5] = kb.is_scancode_pressed(Scancode::Num5) as u8;
        keys[0x6] = kb.is_scancode_pressed(Scancode::Num6) as u8;
        keys[0x7] = kb.is_scancode_pressed(Scancode::Num7) as u8;
        keys[0x8] = kb.is_scancode_pressed(Scancode::Num8) as u8;
        keys[0x9] = kb.is_scancode_pressed(Scancode::Num9) as u8;
        keys[0xA] = kb.is_scancode_pressed(Scancode::A) as u8;
        keys[0xB] = kb.is_scancode_pressed(Scancode::B) as u8;
        keys[0xC] = kb.is_scancode_pressed(Scancode::C) as u8;
        keys[0xD] = kb.is_scancode_pressed(Scancode::D) as u8;
        keys[0xE] = kb.is_scancode_pressed(Scancode::E) as u8;
        keys[0xF] = kb.is_scancode_pressed(Scancode::F) as u8;

        return Command::Continue;
    }
}