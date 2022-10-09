pub(crate) const HISTORY_LENGTH: usize = 20;

pub(crate) const LIFE_REF: [[[usize; 9]; 6]; 12] = [
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
];
pub(crate) const INIT_CHANCE_REF: [usize; LIFE_REF.len()] = [8, 4, 5, 12, 2, 32, 7, 2, 6, 4, 2, 2];

pub(crate) const DELAY_MS: u64 = 80;

pub(crate) const COLOUR_REF: [[usize; 6]; LIFE_REF.len()] = [
    [0, 15, 251, 245, 239, 233],  // game of life
    [0, 17, 18, 19, 20, 21],      // like star wars
    [0, 21, 20, 19, 18, 17],      // walled cities
    [0, 190, 148, 106, 64, 22],   // coagulations
    [0, 51, 45, 39, 33, 20],      // day night (might make the 0 a 17 again)
    [0, 156, 114, 72, 29, 23],    // maze
    [0, 220, 178, 142, 100, 58],  // 2x2
    [0, 84, 172, 82, 30, 18],     // amoeba with diff die time
    [0, 28, 40, 82, 118, 154],    // frogs
    [0, 160, 196, 166, 202, 220], // living on the edge
    [0, 196, 160, 124, 88, 52],   // high life
    [0, 207, 141, 139, 103, 60],  // move
];

pub(crate) const PALETTE: [[u32; 3]; 256] = [
    [0, 0, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 128, 0],
    [0, 0, 128],
    [128, 0, 128],
    [0, 128, 128],
    [192, 192, 192],
    [128, 128, 128],
    [255, 0, 0],
    [0, 255, 0],
    [255, 255, 0],
    [0, 0, 255],
    [255, 0, 255],
    [0, 255, 255],
    [255, 255, 255],
    [0, 0, 0],
    [0, 0, 95],
    [0, 0, 135],
    [0, 0, 175],
    [0, 0, 215],
    [0, 0, 255],
    [0, 95, 0],
    [0, 95, 95],
    [0, 95, 135],
    [0, 95, 175],
    [0, 95, 215],
    [0, 95, 255],
    [0, 135, 0],
    [0, 135, 95],
    [0, 135, 135],
    [0, 135, 175],
    [0, 135, 215],
    [0, 135, 255],
    [0, 175, 0],
    [0, 175, 95],
    [0, 175, 135],
    [0, 175, 175],
    [0, 175, 215],
    [0, 175, 255],
    [0, 215, 0],
    [0, 215, 95],
    [0, 215, 135],
    [0, 215, 175],
    [0, 215, 215],
    [0, 215, 255],
    [0, 255, 0],
    [0, 255, 95],
    [0, 255, 135],
    [0, 255, 175],
    [0, 255, 215],
    [0, 255, 255],
    [95, 0, 0],
    [95, 0, 95],
    [95, 0, 135],
    [95, 0, 175],
    [95, 0, 215],
    [95, 0, 255],
    [95, 95, 0],
    [95, 95, 95],
    [95, 95, 135],
    [95, 95, 175],
    [95, 95, 215],
    [95, 95, 255],
    [95, 135, 0],
    [95, 135, 95],
    [95, 135, 135],
    [95, 135, 175],
    [95, 135, 215],
    [95, 135, 255],
    [95, 175, 0],
    [95, 175, 95],
    [95, 175, 135],
    [95, 175, 175],
    [95, 175, 215],
    [95, 175, 255],
    [95, 215, 0],
    [95, 215, 95],
    [95, 215, 135],
    [95, 215, 175],
    [95, 215, 215],
    [95, 215, 255],
    [95, 255, 0],
    [95, 255, 95],
    [95, 255, 135],
    [95, 255, 175],
    [95, 255, 215],
    [95, 255, 255],
    [135, 0, 0],
    [135, 0, 95],
    [135, 0, 135],
    [135, 0, 175],
    [135, 0, 215],
    [135, 0, 255],
    [135, 95, 0],
    [135, 95, 95],
    [135, 95, 135],
    [135, 95, 175],
    [135, 95, 215],
    [135, 95, 255],
    [135, 135, 0],
    [135, 135, 95],
    [135, 135, 135],
    [135, 135, 175],
    [135, 135, 215],
    [135, 135, 255],
    [135, 175, 0],
    [135, 175, 95],
    [135, 175, 135],
    [135, 175, 175],
    [135, 175, 215],
    [135, 175, 255],
    [135, 215, 0],
    [135, 215, 95],
    [135, 215, 135],
    [135, 215, 175],
    [135, 215, 215],
    [135, 215, 255],
    [135, 255, 0],
    [135, 255, 95],
    [135, 255, 135],
    [135, 255, 175],
    [135, 255, 215],
    [135, 255, 255],
    [175, 0, 0],
    [175, 0, 95],
    [175, 0, 135],
    [175, 0, 175],
    [175, 0, 215],
    [175, 0, 255],
    [175, 95, 0],
    [175, 95, 95],
    [175, 95, 135],
    [175, 95, 175],
    [175, 95, 215],
    [175, 95, 255],
    [175, 135, 0],
    [175, 135, 95],
    [175, 135, 135],
    [175, 135, 175],
    [175, 135, 215],
    [175, 135, 255],
    [175, 175, 0],
    [175, 175, 95],
    [175, 175, 135],
    [175, 175, 175],
    [175, 175, 215],
    [175, 175, 255],
    [175, 215, 0],
    [175, 215, 95],
    [175, 215, 135],
    [175, 215, 175],
    [175, 215, 215],
    [175, 215, 255],
    [175, 255, 0],
    [175, 255, 95],
    [175, 255, 135],
    [175, 255, 175],
    [175, 255, 215],
    [175, 255, 255],
    [215, 0, 0],
    [215, 0, 95],
    [215, 0, 135],
    [215, 0, 175],
    [215, 0, 215],
    [215, 0, 255],
    [215, 95, 0],
    [215, 95, 95],
    [215, 95, 135],
    [215, 95, 175],
    [215, 95, 215],
    [215, 95, 255],
    [215, 135, 0],
    [215, 135, 95],
    [215, 135, 135],
    [215, 135, 175],
    [215, 135, 215],
    [215, 135, 255],
    [215, 175, 0],
    [215, 175, 95],
    [215, 175, 135],
    [215, 175, 175],
    [215, 175, 215],
    [215, 175, 255],
    [215, 215, 0],
    [215, 215, 95],
    [215, 215, 135],
    [215, 215, 175],
    [215, 215, 215],
    [215, 215, 255],
    [215, 255, 0],
    [215, 255, 95],
    [215, 255, 135],
    [215, 255, 175],
    [215, 255, 215],
    [215, 255, 255],
    [255, 0, 0],
    [255, 0, 95],
    [255, 0, 135],
    [255, 0, 175],
    [255, 0, 215],
    [255, 0, 255],
    [255, 95, 0],
    [255, 95, 95],
    [255, 95, 135],
    [255, 95, 175],
    [255, 95, 215],
    [255, 95, 255],
    [255, 135, 0],
    [255, 135, 95],
    [255, 135, 135],
    [255, 135, 175],
    [255, 135, 215],
    [255, 135, 255],
    [255, 175, 0],
    [255, 175, 95],
    [255, 175, 135],
    [255, 175, 175],
    [255, 175, 215],
    [255, 175, 255],
    [255, 215, 0],
    [255, 215, 95],
    [255, 215, 135],
    [255, 215, 175],
    [255, 215, 215],
    [255, 215, 255],
    [255, 255, 0],
    [255, 255, 95],
    [255, 255, 135],
    [255, 255, 175],
    [255, 255, 215],
    [255, 255, 255],
    [8, 8, 8],
    [18, 18, 18],
    [28, 28, 28],
    [38, 38, 38],
    [48, 48, 48],
    [58, 58, 58],
    [68, 68, 68],
    [78, 78, 78],
    [88, 88, 88],
    [98, 98, 98],
    [108, 108, 108],
    [118, 118, 118],
    [128, 128, 128],
    [138, 138, 138],
    [148, 148, 148],
    [158, 158, 158],
    [168, 168, 168],
    [178, 178, 178],
    [188, 188, 188],
    [198, 198, 198],
    [208, 208, 208],
    [218, 218, 218],
    [228, 228, 228],
    [238, 238, 238],
];

//i'll change each of these to an index on some other reference eventually
//that time is now!
//pub(crate) const CHAR_PALETTE: String = [
//    " ·."
//].concat().chars().collect();
//not sure wether to make it a string or array of chars
pub(crate) const CHAR_PALETTE: [char; 20] = [
    ' ', '·', '+', 's', '3', '.', ',', '*', 'S', '4', '%', '&', '$', '#', '5', '@', 'O', '0', 'M',
    '8',
];
//█

/*
     ·+s3
    .,*S4
    %&$#5
    @O0M8
*/

//character index on reference for each state of each lifetype
pub(crate) const CHAR_REF: [[usize; 6]; 12] = [
    /*
    [' ', '#', '$', '%', '+', '·'], // game of life
    [' ', '#', '$', '%', '+', '·'], // like star wars
    [' ', '#', '$', '%', '+', '·'], // walled cities
    [' ', '0', '$', '%', '+', '·'], // edited coagulations
    [' ', '@', '$', '%', '+', '·'], // day night
    [' ', '0', '&', '*', '+', '·'], // maze
    [' ', 'S', 's', '3', '4', '5'],  // 2x2
    [' ', 'O', '$', '%', '+', '·'], // edited amoeba
    [' ', '@', '$', '0', 'O', '*'],  // frogs
    [' ', '#', '$', '%', '+', '·'], // living on the edge
    [' ', '&', '$', '%', '+', '·'], // high life
    [' ', 'M', '$', '%', '+', '·'], // move
    */
    [0, 13, 12, 10, 2, 1],  // game of life
    [0, 13, 12, 10, 2, 1],  // like star wars
    [0, 13, 12, 10, 2, 1],  // walled cities
    [0, 17, 12, 10, 2, 1],  // edited coagulations
    [0, 15, 12, 10, 2, 1],  // day night
    [0, 17, 11, 7, 2, 1],   // maze
    [0, 8, 3, 4, 9, 14],    // 2x2
    [0, 16, 12, 10, 2, 1],  // edited amoeba
    [0, 15, 12, 17, 16, 7], // frogs
    [0, 13, 12, 10, 2, 1],  // living on the edge
    [0, 11, 12, 10, 2, 1],  // high life
    [0, 18, 12, 10, 2, 1],  // move
];
