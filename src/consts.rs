pub(crate) const HISTORY_LENGTH: usize = 20;
pub(crate) const MAX_FUTURE_CHECK: usize = 15; //how many iterations it checks to see what lifetype will extend activity the longest, as well as how many it checks to see if the board is going to be empty
pub(crate) const HISTORY_EMPTY_CHECK: usize = 10;
pub(crate) const DELAY_MS: u64 = 80; //110;
pub(crate) const WIDTH: usize = 80; //100;
pub(crate) const HEIGHT: usize = 49; //58;
                                     //multipliers for the base limited_life_timer init length, including the random portion. the defualt (1) is 10 +- 4 (8 + 1..=4)
pub(crate) const LIFETIME_REF: [usize; 14] = [25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 4, 2];
//150/8
pub(crate) const PERCENTAGE_SIZE_REDUCTION: f64 = 0.89; //percentage of the size of the board pixels that the black mask will be
pub(crate) const PERCENTAGE_ROUNDED_CORNERS: f64 = 0.3; //correlates to how rounded the corners are, 1.0 is a square, 0.0 is a circle
pub(crate) const DETAIL_MULTIPLIER: usize = 20;
pub(crate) const LIFE_REF: [[[usize; 9]; 6]; 14] = [
    // new state for a cell depending on various factors
    [
        //game of life
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //like star wars
        [0, 0, 1, 0, 0, 0, 0, 1, 1],
        [2, 2, 2, 1, 1, 1, 1, 2, 2],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //walled cities
        [0, 0, 0, 0, 1, 1, 1, 1, 1],
        [4, 4, 1, 1, 1, 1, 4, 4, 4],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //[4, 4, 4, 4, 4, 4, 4, 4, 4],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //coagulations (changed a little)
        [0, 0, 0, 1, 0, 0, 0, 1, 1],
        [0, 0, 1, 1, 0, 1, 1, 1, 1], //[2, 3, ...
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 0, 5, 5, 5, 5, 5, 5, 5],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //day night
        [0, 0, 0, 1, 0, 0, 1, 1, 1],
        [0, 0, 0, 1, 1, 0, 1, 1, 1],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //maze
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //2x2
        [0, 0, 0, 1, 0, 0, 1, 0, 0],
        [4, 1, 1, 4, 4, 1, 4, 4, 4],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //amoeba with diff die time
        [0, 0, 0, 1, 0, 1, 0, 1, 0],
        [4, 4, 1, 1, 4, 1, 1, 4, 1],
        [3, 3, 3, 3, 3, 3, 3, 3, 3], // done
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //frogs
        [0, 0, 0, 1, 1, 0, 0, 0, 0],
        [3, 1, 1, 3, 3, 3, 3, 3, 3],
        [4, 4, 4, 4, 4, 4, 4, 4, 4],
        [5, 5, 5, 5, 5, 5, 5, 5, 5], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //living on the edge
        [0, 0, 0, 1, 0, 0, 0, 1, 0],
        [3, 3, 3, 1, 1, 1, 3, 3, 1],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [5, 5, 5, 5, 5, 5, 5, 5, 5], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //high life
        [0, 0, 0, 1, 0, 0, 1, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [4, 4, 4, 4, 4, 4, 4, 4, 4], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //move
        [0, 0, 0, 1, 0, 0, 1, 0, 1],
        [0, 0, 1, 0, 1, 1, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [4, 4, 4, 4, 4, 4, 4, 4, 4], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //seed (B2/S)
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [4, 4, 4, 4, 4, 4, 4, 4, 4], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        //gnarl (B1/S1)
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [4, 4, 4, 4, 4, 4, 4, 4, 4], //done
        [5, 5, 5, 5, 5, 5, 5, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
];
pub(crate) const INIT_CHANCE_REF: [usize; LIFE_REF.len()] =
    [8, 4, 12, 22, 2, 20, 7, 2, 6, 4, 2, 2, 20, 23];
pub(crate) const COLOUR_REF: [[u32; 6]; LIFE_REF.len()] = [
    [0x000000, 0xffffff, 0xc6c6c6, 0x8a8a8a, 0x4e4e4e, 0x121212], // game of life
    [0x000000, 0x00005f, 0x000087, 0x0000af, 0x0000d7, 0x0000ff], // like star wars
    [0x000000, 0x2e2e5e, 0x00ffff, 0x00d7ff, 0x00afff, 0x0087ff], // walled cities
    [0x000000, 0xd7ff00, 0xafd700, 0x87af00, 0x5f8700, 0x005f00], // coagulations
    [0x000000, 0x00ffff, 0x00d7ff, 0x00afff, 0x0087ff, 0x0000d7], // day night (might make the 0 a 17 again)
    [0x000000, 0xafff87, 0x87d787, 0x5faf87, 0x875f00, 0x5f5f00], // maze
    [0x000000, 0xffd700, 0xd7af00, 0xafaf00, 0x878700, 0x5f5f00], // 2x2
    [0x000000, 0x5fff87, 0x00d787, 0x005fff, 0x008787, 0x000087], // amoeba with diff die time
    [0x000000, 0x008700, 0x00d700, 0x5fff00, 0x87ff00, 0xafff00], // frogs
    [0x000000, 0xd70000, 0xff0000, 0xd75f00, 0xff5f00, 0xffd700], // living on the edge
    [0x000000, 0xff0000, 0xd70000, 0xaf0000, 0x870000, 0x5f0000], // high life
    [0x000000, 0x0093ff, 0x006ebf, 0x00518d, 0x00355d, 0x00213b], // move
    [0x000000, 0xddffdd, 0xc2d7c2, 0x88af88, 0x448744, 0x3f5f3f], // seed
    [0x000000, 0xffef00, 0xd7c700, 0xaf9f00, 0x877700, 0x5f5000], // gnarl
];
