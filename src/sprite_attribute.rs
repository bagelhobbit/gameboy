use crate::util::get_as_bits;

#[derive(Debug)]
pub struct SpriteAttribute {
    pub y: u8,
    pub x: u8,
    pub index: u8,
    pub bg_over_obj: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: u8,
}

impl SpriteAttribute {
    pub fn new(y: u8, x: u8, index: u8, flags: u8) -> SpriteAttribute {
        let flag_bits = get_as_bits(flags);
        let bg_over_obj = flag_bits[0] == 1;
        let y_flip = flag_bits[1] == 1;
        let x_flip = flag_bits[2] == 1;
        let palette = flag_bits[3];
        SpriteAttribute {
            y,
            x,
            index,
            bg_over_obj,
            y_flip,
            x_flip,
            palette,
        }
    }
}
