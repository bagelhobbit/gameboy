use sdl2::keyboard::Keycode;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    None,
    Action,
    Direction,
}

#[derive(Debug, Default)]
pub struct Joypad {
    pub selected_buttons: ButtonType,
    down_pressed: bool,
    up_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    start_pressed: bool,
    select_pressed: bool,
    b_pressed: bool,
    a_pressed: bool,
}

impl Joypad {
    pub fn set_inputs(&mut self, pressed_keys: HashSet<Keycode>) {
        self.reset_buttons();

        if pressed_keys.contains(&Keycode::Down) {
            self.down_pressed = true;
        }

        if pressed_keys.contains(&Keycode::Up) {
            self.up_pressed = true;
        }

        if pressed_keys.contains(&Keycode::Left) {
            self.left_pressed = true;
        }

        if pressed_keys.contains(&Keycode::Right) {
            self.right_pressed = true;
        }

        if pressed_keys.contains(&Keycode::Return) {
            self.start_pressed = true;
        }

        if pressed_keys.contains(&Keycode::RShift) || pressed_keys.contains(&Keycode::LShift) {
            self.select_pressed = true;
        }

        if pressed_keys.contains(&Keycode::A) {
            self.b_pressed = true;
        }

        if pressed_keys.contains(&Keycode::S) {
            self.a_pressed = true;
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self.selected_buttons {
            ButtonType::Action => {
                let binary_string = format!(
                    "1101{}{}{}{}",
                    (!self.start_pressed) as u8,
                    (!self.select_pressed) as u8,
                    (!self.b_pressed) as u8,
                    (!self.a_pressed) as u8
                );
                u8::from_str_radix(&binary_string, 2).unwrap()
            }
            ButtonType::Direction => {
                let binary_string = format!(
                    "1110{}{}{}{}",
                    (!self.down_pressed) as u8,
                    (!self.up_pressed) as u8,
                    (!self.left_pressed) as u8,
                    (!self.right_pressed) as u8
                );
                u8::from_str_radix(&binary_string, 2).unwrap()
            }
            ButtonType::None => 0xFF,
        }
    }

    fn reset_buttons(&mut self) {
        self.down_pressed = false;
        self.up_pressed = false;
        self.left_pressed = false;
        self.right_pressed = false;
        self.start_pressed = false;
        self.select_pressed = false;
        self.b_pressed = false;
        self.a_pressed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_byte_action() {
        let mut joypad = Joypad::default();
        joypad.selected_buttons = ButtonType::Action;

        assert_eq!(joypad.as_byte(), 0b1101_1111, "No Buttons");

        joypad.start_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1101_0111, "Start button");

        joypad.select_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1101_0011, "Select button");

        joypad.b_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1101_0001, "B button");

        joypad.a_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1101_0000, "A button");
    }

    #[test]
    fn test_as_byte_direction() {
        let mut joypad = Joypad::default();
        joypad.selected_buttons = ButtonType::Direction;

        assert_eq!(joypad.as_byte(), 0b1110_1111, "No Buttons");

        joypad.down_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1110_0111, "Down button");

        joypad.up_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1110_0011, "Up button");

        joypad.left_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1110_0001, "Left button");

        joypad.right_pressed = true;
        assert_eq!(joypad.as_byte(), 0b1110_0000, "Right button");
    }

    #[test]
    fn test_as_byte_none() {
        let joypad = Joypad::default();

        assert_eq!(joypad.as_byte(), 0xFF);
    }
}
