use crate::util::get_as_bits;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TileType {
    Background,
    Window,
    Obj,
}

#[derive(PartialEq, Eq)]
pub struct TileInfo {
    pub tile: [u8; 16],
    pub tile_type: TileType,
}

impl fmt::Debug for TileInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = "".to_owned();
        let mut tile_colors = "".to_owned();

        for byte in self.tile {
            bytes.push_str(format!("{:0>2X}, ", byte).as_str());
        }

        let colors = self.get_color_ids_from_tile();
        for row in 0..colors.len() {
            for col in 0..colors[0].len() {
                tile_colors.push_str(format!("{} ", colors[row][col]).as_str());
            }
            tile_colors.push('\n');
        }

        write!(
            f,
            "TileInfo {{ tile: [{}], tile_type: {:?} }}\n{}",
            bytes, self.tile_type, tile_colors
        )
    }
}

impl TileInfo {
    pub fn get_color_ids_from_tile(&self) -> [[u8; 8]; 8] {
        let mut result = [[0; 8]; 8];

        for (index, row) in (0..self.tile.len()).step_by(2).enumerate() {
            let lsb = get_as_bits(self.tile[row]);
            let msb = get_as_bits(self.tile[row + 1]);

            for bit in 0..lsb.len() {
                result[index][bit] = (msb[bit] << 1) + lsb[bit];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_basic_tile_info() -> TileInfo {
        let tile = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];

        let tile_info = TileInfo {
            tile,
            tile_type: TileType::Obj,
        };

        tile_info
    }

    #[test]
    fn test_get_color_ids_from_tile() {
        // First byte is the LSB, second byte is MSB
        // combined these bytes encode the color ids for a tile
        //
        // tile[0] = 0x3C; //0b0011_1100
        // tile[0] = 0x7E; //0b0111_1110
        // result => 0b00, 0b10, 0b11, 0b11, 0b11, 0b11, 0b10, 0b00

        let colors = get_basic_tile_info().get_color_ids_from_tile();

        assert_eq!(colors[0], [0, 2, 3, 3, 3, 3, 2, 0]);
        assert_eq!(colors[1], [0, 3, 0, 0, 0, 0, 3, 0]);
        assert_eq!(colors[2], [0, 3, 0, 0, 0, 0, 3, 0]);
        assert_eq!(colors[3], [0, 3, 0, 0, 0, 0, 3, 0]);
        assert_eq!(colors[4], [0, 3, 1, 3, 3, 3, 3, 0]);
        assert_eq!(colors[5], [0, 1, 1, 1, 3, 1, 3, 0]);
        assert_eq!(colors[6], [0, 3, 1, 3, 1, 3, 2, 0]);
        assert_eq!(colors[7], [0, 2, 3, 3, 3, 2, 0, 0]);
    }
}
