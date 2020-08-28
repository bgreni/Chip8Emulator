use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Inputs {
    events: EventPump,
}

impl Inputs {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        return Inputs { events: sdl_context.event_pump().unwrap() };
    }
    pub fn poll(&mut self) -> PollResult {
        let keys = self.check_key_presses();
        let should_quit = self.check_quit();
        return PollResult {
            keys_pressed: keys,
            should_quit: should_quit
        }
    }
    pub fn check_quit(&mut self) -> bool {
        for event in self.events.poll_iter() {
            match event {
                  Event::Quit {..} => return true,
                _ => {},
            }
        }
        return false;
    }

    pub fn check_key_presses(&self) -> [bool; 16] {
        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }
        return chip8_keys;
    }
 }

pub fn check_keys(keys: &[bool; 16]) -> Option<u8> {
    for i in 0..keys.len() {
        if keys[i] == true {
            return Some(i as u8);
        }
    }
    return None;
}

pub fn val_in_keys(keys: &[bool; 16], target_key: usize) -> bool {
    return keys[target_key] == true;
}


pub struct PollResult {
    pub keys_pressed: [bool; 16],
    pub should_quit: bool
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn test_check_keys() {
        let mut keys = [false; 16];
        keys[4] = true;
        let result = check_keys(&keys);
        assert_eq!(result, Some(4));
    }
}
