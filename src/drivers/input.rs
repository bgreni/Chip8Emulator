use piston::input::Button;
use piston::input::keyboard::Key;

pub struct Inputs {
}

impl Inputs {
    pub fn new() -> Self {
        return Inputs{};
    }
    // pub fn check_quit(&mut self) -> bool {
    //     for event in self.events.poll_iter() {
    //         match event {
    //               Event::Quit {..} => return true,
    //             _ => {},
    //         }
    //     }
    //     return false;
    // }

    pub fn check_key_presses(&self, press: Option<Button>) -> Option<u8> {
        if press == None {
            return None;
        }
        return match press.unwrap() {
            Button::Keyboard(Key::NumPad1) => Some(0x0),
            Button::Keyboard(Key::NumPad2) => Some(0x1),
            Button::Keyboard(Key::NumPad3) => Some(0x2),
            Button::Keyboard(Key::NumPad4) => Some(0x3),
            Button::Keyboard(Key::Q) => Some(0x4),
            Button::Keyboard(Key::W) => Some(0x5),
            Button::Keyboard(Key::E) => Some(0x6),
            Button::Keyboard(Key::R) => Some(0x7),
            Button::Keyboard(Key::A) => Some(0x8),
            Button::Keyboard(Key::S) => Some(0x9),
            Button::Keyboard(Key::D) => Some(0xa),
            Button::Keyboard(Key::F) => Some(0xb),
            Button::Keyboard(Key::Z) => Some(0xc),
            Button::Keyboard(Key::X) => Some(0xd),
            Button::Keyboard(Key::C) => Some(0xe),
            Button::Keyboard(Key::V) => Some(0xf),
            _ => { None },
        }
    }
 }

// pub fn check_keys(keys: &[bool; 16]) -> Option<u8> {
//     for i in 0..keys.len() {
//         if keys[i] == true {
//             return Some(i as u8);
//         }
//     }
//     return None;
// }

// pub fn val_in_keys(keys: &[bool; 16], target_key: usize) -> bool {
//     return keys[target_key] == true;
// }


pub struct PollResult {
    pub keys_pressed: [bool; 16],
    pub should_quit: bool
}

// #[cfg(test)]
// mod input_tests {
//     use super::*;
//
//     #[test]
//     fn test_check_keys() {
//         let mut keys = [false; 16];
//         keys[4] = true;
//         let result = check_key_presses(&keys);
//         assert_eq!(result, Some(4));
//     }
// }
