use sdl2::rect::Rect;

use crate::{
    memory::Memory,
    tile_info::{TileInfo, TileType},
};

#[derive(Debug)]
pub struct Ppu {
    pub pixel_width: u32,
    pub pixel_height: u32,
}

#[derive(Debug, Default)]
pub struct ColorRects {
    pub color0_rects: Vec<Rect>,
    pub color1_rects: Vec<Rect>,
    pub color2_rects: Vec<Rect>,
    pub color3_rects: Vec<Rect>,
    pub transparent_rects: Vec<Rect>,
}

impl Ppu {
    pub fn render_scanline(
        &self,
        memory: &Memory,
        y: usize,
        tilemap: &[[u8; 32]; 32],
        color_values: &[u8; 4],
        color_rects: &mut ColorRects,
    ) {
        // Render an extra tile for smooth scrolling
        for x in 0..21 {
            let tile = memory.vram_read_tile(
                TileType::Background,
                tilemap[((memory.scy as usize + y) / 8) % 32]
                    [((memory.scx as usize + (8 * x)) / 8) % 32],
            );

            let x_pos = x as i32 * 8;
            let x_offset = memory.scx as i32 % 8;

            self.get_rects_for_tile(
                tile,
                x_pos - x_offset,
                y as i32,
                memory.scy as i32 % 8,
                color_values,
                color_rects,
            );
        }
    }

    pub fn render_window_scanline(
        &self,
        memory: &Memory,
        y: usize,
        tilemap: &[[u8; 32]; 32],
        color_values: &[u8; 4],
        color_rects: &mut ColorRects,
    ) {
        let y_tile_index = if y >= memory.wy as usize {
            y - memory.wy as usize
        } else {
            0
        };

        for x in 0..20 {
            let tile = memory.vram_read_tile(
                TileType::Window,
                tilemap[(y_tile_index / 8) % 32][(x / 8) % 32],
            );

            let x_pos = x as i32 * 8;

            self.get_rects_for_tile(
                tile,
                x_pos,
                y as i32,
                0,
                color_values,
                color_rects,
            );
        }
    }

    fn get_rects_for_tile(
        &self,
        tile: TileInfo,
        tile_start: i32,
        line: i32,
        y_offset: i32,
        color_values: &[u8; 4],
        color_rects: &mut ColorRects,
    ) {
        let line_colors = get_row_from_tile(tile, line + y_offset);

        for col in 0..line_colors.len() {
            let rect = Rect::new(
                (tile_start + col as i32) * self.pixel_width as i32,
                line * self.pixel_height as i32,
                self.pixel_width,
                self.pixel_height,
            );

            let palette = color_values[line_colors[col] as usize];

            if palette == 0 {
                color_rects.color0_rects.push(rect);
            } else if palette == 1 {
                color_rects.color1_rects.push(rect);
            } else if palette == 2 {
                color_rects.color2_rects.push(rect);
            } else {
                color_rects.color3_rects.push(rect);
            }
        }
    }
}

fn get_row_from_tile(tile: TileInfo, line: i32) -> [u8; 8] {
    let colors = tile.get_color_ids_from_tile();

    colors[(line as usize % 8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_TILE_COLORS: [[u8; 8]; 8] = [
        [0, 2, 3, 3, 3, 3, 2, 0],
        [0, 3, 0, 0, 0, 0, 3, 0],
        [0, 3, 0, 0, 0, 0, 3, 0],
        [0, 3, 0, 0, 0, 0, 3, 0],
        [0, 3, 1, 3, 3, 3, 3, 0],
        [0, 1, 1, 1, 3, 1, 3, 0],
        [0, 3, 1, 3, 1, 3, 2, 0],
        [0, 2, 3, 3, 3, 2, 0, 0],
    ];

    #[rustfmt::skip]
    const BASIC_TILE: TileInfo = TileInfo {
        tile: [
            0x3C, 0x7E, 
            0x42, 0x42, 
            0x42, 0x42, 
            0x42, 0x42, 
            0x7E, 0x5E, 
            0x7E, 0x0A, 
            0x7C, 0x56,
            0x38, 0x7C,
        ],
        tile_type: TileType::Background,
    };

    #[test]
    fn test_get_scanline_from_tile_under_8() {
        for i in 0..8 {
            assert_eq!(
                BASIC_TILE_COLORS[i],
                get_row_from_tile(BASIC_TILE, i as i32),
                "failed on row {}",
                i
            )
        }
    }

    #[test]
    fn test_get_scanline_from_tile_over_8() {
        for i in 0..8 {
            assert_eq!(
                BASIC_TILE_COLORS[i],
                get_row_from_tile(BASIC_TILE, i as i32 + 8),
                "failed on row {}",
                i
            )
        }
    }

    #[test]
    fn test_get_rects_for_tile_line_0() {
        let ppu = Ppu {
            pixel_width: 2,
            pixel_height: 2,
        };
        let mut color_rects = ColorRects::default();
        ppu.get_rects_for_tile(BASIC_TILE, 16, 16, 0, &[0, 1, 2, 3], &mut color_rects);

        assert_eq!(
            vec![
                Rect::new(16 * 2, 16 * 2, 2, 2),
                Rect::new(23 * 2, 16 * 2, 2, 2)
            ],
            color_rects.color0_rects
        )
    }

    #[test]
    fn test_get_rects_for_tile_line_5() {
        let ppu = Ppu {
            pixel_width: 2,
            pixel_height: 2,
        };
        let mut color_rects = ColorRects::default();
        ppu.get_rects_for_tile(BASIC_TILE, 16, 20, 0, &[0, 1, 2, 3], &mut color_rects);

        assert_eq!(
            vec![
                Rect::new(16 * 2, 20 * 2, 2, 2),
                Rect::new(23 * 2, 20 * 2, 2, 2)
            ],
            color_rects.color0_rects
        )
    }
}
