use crate::{HEIGHT, WIDTH};

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn u32_to_u8_rgb(colour: u32) -> (u8, u8, u8) {
    let r = (colour >> 16) as u8;
    let g = (colour >> 8) as u8;
    let b = colour as u8;
    (r, g, b)
}

pub(crate) fn print_board(
    board: &Vec<Vec<usize>>,
    old_board: &Vec<Vec<usize>>,
    colours: [u32; 6],
    old_colours: [u32; 6],
    buffer: &mut Vec<u32>,
    buffer_width: usize,
    buffer_height: usize,
    black_mask: &Vec<bool>,
    blend: f32, //0.0 to 1.0, determines how much of the old colour to keep
) {
    //make the blended colour palette of palettes
    let mut blended_colours = [[0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let new_colour = u32_to_u8_rgb(colours[i]);
            let old_colour = u32_to_u8_rgb(old_colours[j]);
            blended_colours[i][j] = from_u8_rgb(
                (new_colour.0 as f32 * blend + old_colour.0 as f32 * (1.0 - blend)) as u8,
                (new_colour.1 as f32 * blend + old_colour.1 as f32 * (1.0 - blend)) as u8,
                (new_colour.2 as f32 * blend + old_colour.2 as f32 * (1.0 - blend)) as u8,
            );
        }
    }
    for buffer_y in 0..buffer_height {
        for buffer_x in 0..buffer_width {
            let board_x = buffer_x * WIDTH / buffer_width;
            let board_y = buffer_y * HEIGHT / buffer_height;

            if black_mask[buffer_y * buffer_width + buffer_x] {
                buffer[buffer_y * buffer_width + buffer_x] = 0x000000;
            } else {
                let colour = blended_colours[board[board_y][board_x]][old_board[board_y][board_x]];
                buffer[buffer_y * buffer_width + buffer_x] = colour as u32;
            }
        }
    }
}
