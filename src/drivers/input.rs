use piston::input::Button;
use piston::input::keyboard::Key;

pub fn keymap(k: Option<Button>) -> Option<u8> {
    if let Some(Button::Keyboard(k)) = k {
        return match k {
            Key::D1 => Some(0x1),
            Key::D2 => Some(0x2),
            Key::D3 => Some(0x3),

            Key::Q  => Some(0x4),
            Key::W  => Some(0x5),
            Key::E  => Some(0x6),

            Key::A  => Some(0x7),
            Key::S  => Some(0x8),
            Key::D  => Some(0x9),

            Key::Z  => Some(0xA),
            Key::X  => Some(0x0),
            Key::C  => Some(0xB),

            Key::D4 => Some(0xC),
            Key::R  => Some(0xD),
            Key::F  => Some(0xE),
            Key::V  => Some(0xF),

            _ => None
        }
    }
    return None
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
