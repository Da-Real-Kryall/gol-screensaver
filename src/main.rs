use minifb::{Window, WindowOptions}; //{Key,
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

pub(crate) mod consts;
pub(crate) mod display;
pub(crate) mod iterate;
use crate::consts::*;
use crate::display::*;
use crate::iterate::*;

fn generate_black_mask(black_mask: &mut Vec<bool>, buffer_width: usize, buffer_height: usize) {
    //this is a mask that makes the drawn pixels rounded, slightly seperated squares.
    //it's a vector of booleans
    //true is black (i think)

    let width_of_board_pixels = buffer_width / WIDTH;
    let height_of_board_pixels = buffer_height / HEIGHT;

    for board_y in 0..HEIGHT {
        for board_x in 0..WIDTH {
            let board_pixel_center_x = board_x * width_of_board_pixels + width_of_board_pixels / 2;
            let board_pixel_center_y =
                board_y * height_of_board_pixels + height_of_board_pixels / 2;
            let radius = width_of_board_pixels as f64 * PERCENTAGE_SIZE_REDUCTION / 2.0;

            let circle_center_distance_from_board_pixel_center =
                radius * PERCENTAGE_ROUNDED_CORNERS;

            //iterate over the pixels in the section of the screen for the current board pixel
            for buffer_y in
                (board_y * height_of_board_pixels)..((board_y + 1) * height_of_board_pixels)
            {
                for buffer_x in
                    (board_x * width_of_board_pixels)..((board_x + 1) * width_of_board_pixels)
                {
                    let distance = (board_pixel_center_x as f64 - buffer_x as f64)
                        .abs()
                        .max((board_pixel_center_y as f64 - buffer_y as f64).abs());
                    if distance > radius {
                        black_mask[buffer_y * buffer_width + buffer_x] = true;
                    } else {
                        black_mask[buffer_y * buffer_width + buffer_x] = false;
                    }
                    //top left corner
                    let tl_circle_center_x = board_pixel_center_x as f64
                        - circle_center_distance_from_board_pixel_center;
                    let tl_circle_center_y = board_pixel_center_y as f64
                        - circle_center_distance_from_board_pixel_center;
                    let tl_distance = ((tl_circle_center_x - buffer_x as f64).powi(2)
                        + (tl_circle_center_y - buffer_y as f64).powi(2))
                    .sqrt();
                    //if the current pixel is outside the circle and to the top left of the circle, make the pixel black
                    if tl_distance > (1.0 - PERCENTAGE_ROUNDED_CORNERS) * radius
                        && buffer_x < tl_circle_center_x as usize
                        && buffer_y < tl_circle_center_y as usize
                    {
                        black_mask[buffer_y * buffer_width + buffer_x] = true;
                    }
                    //top right corner
                    let tr_circle_center_x = board_pixel_center_x as f64
                        + circle_center_distance_from_board_pixel_center;
                    let tr_circle_center_y = board_pixel_center_y as f64
                        - circle_center_distance_from_board_pixel_center;
                    let tr_distance = ((tr_circle_center_x - buffer_x as f64).powi(2)
                        + (tr_circle_center_y - buffer_y as f64).powi(2))
                    .sqrt();
                    //if the current pixel is outside the circle and to the top right of the circle, make the pixel black
                    if tr_distance > (1.0 - PERCENTAGE_ROUNDED_CORNERS) * radius
                        && buffer_x > tr_circle_center_x as usize
                        && buffer_y < tr_circle_center_y as usize
                    {
                        black_mask[buffer_y * buffer_width + buffer_x] = true;
                    }
                    //bottom left corner
                    let bl_circle_center_x = board_pixel_center_x as f64
                        - circle_center_distance_from_board_pixel_center;
                    let bl_circle_center_y = board_pixel_center_y as f64
                        + circle_center_distance_from_board_pixel_center;
                    let bl_distance = ((bl_circle_center_x - buffer_x as f64).powi(2)
                        + (bl_circle_center_y - buffer_y as f64).powi(2))
                    .sqrt();
                    //if the current pixel is outside the circle and to the bottom left of the circle, make the pixel black
                    if bl_distance > (1.0 - PERCENTAGE_ROUNDED_CORNERS) * radius
                        && buffer_x < bl_circle_center_x as usize
                        && buffer_y > bl_circle_center_y as usize
                    {
                        black_mask[buffer_y * buffer_width + buffer_x] = true;
                    }
                    //bottom right corner
                    let br_circle_center_x = board_pixel_center_x as f64
                        + circle_center_distance_from_board_pixel_center;
                    let br_circle_center_y = board_pixel_center_y as f64
                        + circle_center_distance_from_board_pixel_center;
                    let br_distance = ((br_circle_center_x - buffer_x as f64).powi(2)
                        + (br_circle_center_y - buffer_y as f64).powi(2))
                    .sqrt();
                    //if the current pixel is outside the circle and to the bottom right of the circle, make the pixel black
                    if br_distance > (1.0 - PERCENTAGE_ROUNDED_CORNERS) * radius
                        && buffer_x > br_circle_center_x as usize
                        && buffer_y > br_circle_center_y as usize
                    {
                        black_mask[buffer_y * buffer_width + buffer_x] = true;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut state_history: VecDeque<Vec<Vec<usize>>> =
        VecDeque::from(vec![
            vec![vec![0; WIDTH as usize]; HEIGHT as usize];
            HISTORY_LENGTH
        ]);
    let mut type_history: VecDeque<usize> = VecDeque::from(vec![0; HISTORY_LENGTH]);
    let mut limited_life_timer: usize = 0;

    let mut colour_palette: [u32; 6];
    let mut old_colour_palette: [u32; 6];

    let window_width = WIDTH * DETAIL_MULTIPLIER;
    let window_height = HEIGHT * DETAIL_MULTIPLIER;
    let buffer_width = window_width;
    let buffer_height = window_height;

    let options = WindowOptions {
        resize: true,
        ..WindowOptions::default()
    };
    let mut window = Window::new("Screensaver", window_width, window_height, options)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut black_mask: Vec<bool> = vec![false; buffer_width * buffer_height];
    generate_black_mask(&mut black_mask, buffer_width, buffer_height);

    let mut current: Instant;
    let mut buffer: Vec<u32> = vec![0; (buffer_width * buffer_height) as usize];
    let mut prev_delay: u64 = DELAY_MS / 2;

    //refill board
    refill_board(&state_history[1], &5);

    while window.is_open() {
        // && !window.is_key_down(Key::Escape)

        let mut new_board = state_history[0].clone();
        let mut new_type: usize = type_history[0];

        match check_board_history(&state_history, &type_history, &limited_life_timer) {
            //0: board is empty, refill it and change lifetype
            //1: board is identical to one of the last 16 boards, change lifetype
            //2: board has been the same for a while, change lifetype
            //3: board is GOING to be empty, change lifetype to one that will not be empty
            0 => {
                new_type = rng.gen_range(0..LIFE_REF.len());
                limited_life_timer =
                    8 * LIFETIME_REF[new_type] + rng.gen_range(0..4) * LIFETIME_REF[new_type]; //reset limited_life_timer

                //previous board is empty, refill after doing all the other stuff first
                new_board = refill_board(&new_board, &type_history[0]); //see if i can remove this clone later
            }
            1 | 2 => {
                let mut shuffled_range = (0..LIFE_REF.len()).collect::<Vec<usize>>();
                shuffled_range.shuffle(&mut rng);
                let mut max_lifetime: usize = 0;
                let mut max_lifetype: usize = shuffled_range[0];
                for lifetype in shuffled_range {
                    //see how many iterations before the board is empty
                    let mut board = new_board.clone();
                    for lifetime in 0..MAX_FUTURE_CHECK {
                        board = update_board(&board, &lifetype, &(lifetype != type_history[0]));
                        if !board.iter().flatten().all(|&x| x != 1) && lifetime > max_lifetime {
                            max_lifetime = lifetime;
                            max_lifetype = lifetype;
                            break;
                        }
                    }
                }
                new_type = max_lifetype;

                limited_life_timer =
                    8 * LIFETIME_REF[new_type] + rng.gen_range(0..4) * LIFETIME_REF[new_type]; //reset limited_life_timer

                //This is to prevent loops where the board doesn't change as no lifetypes will save it from death.
                if &state_history[0] == &state_history[HISTORY_LENGTH - 1] {
                    new_board = refill_board(&new_board, &type_history[0]);
                }
            }
            3 => {
                let mut shuffled_range = (0..LIFE_REF.len()).collect::<Vec<usize>>();
                shuffled_range.shuffle(&mut rng);
                let mut max_lifetime: usize = 0;
                let mut max_lifetype: usize = shuffled_range[0];
                for lifetype in shuffled_range {
                    //see how many iterations before the board is empty
                    let mut board = new_board.clone();
                    for lifetime in 0..MAX_FUTURE_CHECK {
                        board = update_board(&board, &lifetype, &(lifetype != type_history[0]));
                        if !board.iter().flatten().all(|&x| x != 1) && lifetime > max_lifetime {
                            max_lifetime = lifetime;
                            max_lifetype = lifetype;
                            break;
                        }
                    }
                }
                new_type = max_lifetype;

                limited_life_timer =
                    8 * LIFETIME_REF[new_type] + rng.gen_range(0..4) * LIFETIME_REF[new_type];
                //reset limited_life_timer
            }
            _ => {}
        }
        //update the board
        new_board = update_board(
            &new_board,
            &type_history[0],
            &(new_type != type_history[0]), // && new_type - 3 < 3),
        ); //see if i can remove this clone later

        //update the history and limited life timer
        state_history.pop_back();
        state_history.push_front(new_board);

        type_history.pop_back();
        type_history.push_front(new_type);

        limited_life_timer -= 1;
        colour_palette = COLOUR_REF[type_history[0]];
        old_colour_palette = COLOUR_REF[type_history[1]];

        //if window.is_key_down(Key::Space) {
        //    limited_life_timer = 0;
        //}

        for blend in 0..3 {
            current = Instant::now();
            print_board(
                &state_history[0],
                &state_history[1],
                colour_palette,
                old_colour_palette,
                &mut buffer,
                buffer_width,
                buffer_height,
                &black_mask,
                match blend {
                    0 => 0.0,
                    1 => 0.33,
                    2 => 0.67,
                    _ => 0.0,
                },
            );
            window
                .update_with_buffer(&buffer, buffer_width, buffer_height)
                .unwrap();
            let duration = current.elapsed();
            let delay = ((DELAY_MS / 2 - duration.as_millis() as u64)
                .max(1)
                .min(DELAY_MS / 2)
                + prev_delay)
                / 2;
            thread::sleep(Duration::from_millis(delay));
            prev_delay = delay;
        }
    }
}
