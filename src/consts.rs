use crate::*;

pub const fn cells_in_house(square: usize) -> [u8; 20] {
    let column_start = square % 9;
    let row_start = square - column_start;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    let mut squares_to_change: u128 = 0;
    squares_to_change |= ((1 << 9) - 1) << row_start;
    squares_to_change |= (1
        + (1 << 9)
        + (1 << 18)
        + (1 << 27)
        + (1 << 36)
        + (1 << 45)
        + (1 << 54)
        + (1 << 63)
        + (1 << 72))
        << column_start;
    squares_to_change |= (0b111 + (0b111 << 9) + (0b111 << 18)) << box_start;
    squares_to_change &= !(1 << square);
    let mut squares_to_change_array = [0; 20];
    squares_to_change_array[0] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[1] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[2] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[3] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[4] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[5] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[6] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[7] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[8] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[9] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[10] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[11] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[12] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[13] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[14] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[15] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[16] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[17] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[18] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[19] = squares_to_change.trailing_zeros() as u8;

    //For when while in const is stablized
    /*let mut i = 0;
    while i < 20 {
        squares_to_change_array[i] = get_last_digit!(squares_to_change, u8);
        i += 1;
    }*/
    squares_to_change_array
}

pub const CELLS_TO_CHANGE: [[u8; 20]; 81] = {
    [
        cells_in_house(0),
        cells_in_house(1),
        cells_in_house(2),
        cells_in_house(3),
        cells_in_house(4),
        cells_in_house(5),
        cells_in_house(6),
        cells_in_house(7),
        cells_in_house(8),
        cells_in_house(9),
        cells_in_house(10),
        cells_in_house(11),
        cells_in_house(12),
        cells_in_house(13),
        cells_in_house(14),
        cells_in_house(15),
        cells_in_house(16),
        cells_in_house(17),
        cells_in_house(18),
        cells_in_house(19),
        cells_in_house(20),
        cells_in_house(21),
        cells_in_house(22),
        cells_in_house(23),
        cells_in_house(24),
        cells_in_house(25),
        cells_in_house(26),
        cells_in_house(27),
        cells_in_house(28),
        cells_in_house(29),
        cells_in_house(30),
        cells_in_house(31),
        cells_in_house(32),
        cells_in_house(33),
        cells_in_house(34),
        cells_in_house(35),
        cells_in_house(36),
        cells_in_house(37),
        cells_in_house(38),
        cells_in_house(39),
        cells_in_house(40),
        cells_in_house(41),
        cells_in_house(42),
        cells_in_house(43),
        cells_in_house(44),
        cells_in_house(45),
        cells_in_house(46),
        cells_in_house(47),
        cells_in_house(48),
        cells_in_house(49),
        cells_in_house(50),
        cells_in_house(51),
        cells_in_house(52),
        cells_in_house(53),
        cells_in_house(54),
        cells_in_house(55),
        cells_in_house(56),
        cells_in_house(57),
        cells_in_house(58),
        cells_in_house(59),
        cells_in_house(60),
        cells_in_house(61),
        cells_in_house(62),
        cells_in_house(63),
        cells_in_house(64),
        cells_in_house(65),
        cells_in_house(66),
        cells_in_house(67),
        cells_in_house(68),
        cells_in_house(69),
        cells_in_house(70),
        cells_in_house(71),
        cells_in_house(72),
        cells_in_house(73),
        cells_in_house(74),
        cells_in_house(75),
        cells_in_house(76),
        cells_in_house(77),
        cells_in_house(78),
        cells_in_house(79),
        cells_in_house(80),
    ]
    //For when while in const is stablized
    /*let mut data = [[0; 20]; 81];
    let mut i = 0;
    while i < 81 {
        data[i] = cells_in_house(i);
        i += 1;
    }
    data*/
};

/** Max 9 bit number */
pub const SUDOKU_MAX: u16 = (1 << 9) - 1;

pub const SOLVED_SUDOKU: u128 = (1 << 81) - 1;

/*
After solving this many squares, do not use pointing pairs
*/
pub const SCANNING_CUTOFF: u32 = 40;
