use bevy::prelude::*;
use array2d::Array2D;
use bitvec::prelude::*;
use rand::prelude::*;

const ROW_COUNT:u8 = 9;
const COL_COUNT:u8 = 9;
const FIELDS_COUNT:usize = 81;
const FIELDS:[(u8, u8); FIELDS_COUNT] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8),
    (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8),
    (2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (2,7), (2,8),
    (3,0), (3,1), (3,2), (3,3), (3,4), (3,5), (3,6), (3,7), (3,8),
    (4,0), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (4,7), (4,8),
    (5,0), (5,1), (5,2), (5,3), (5,4), (5,5), (5,6), (5,7), (5,8),
    (6,0), (6,1), (6,2), (6,3), (6,4), (6,5), (6,6), (6,7), (6,8),
    (7,0), (7,1), (7,2), (7,3), (7,4), (7,5), (7,6), (7,7), (7,8),
    (8,0), (8,1), (8,2), (8,3), (8,4), (8,5), (8,6), (8,7), (8,8),
];
const QUADS:[[u8;9];9] = [
    [0,0,0,1,1,1,2,2,2,],
    [0,0,0,1,1,1,2,2,2,],
    [0,0,0,1,1,1,2,2,2,],
    [3,3,3,4,4,4,5,5,5,],
    [3,3,3,4,4,4,5,5,5,],
    [3,3,3,4,4,4,5,5,5,],
    [6,6,6,7,7,7,8,8,8,],
    [6,6,6,7,7,7,8,8,8,],
    [6,6,6,7,7,7,8,8,8,],
];

const VALUES_BIN:[u16;9] = [1,2,4,8,16,32,64,128,256];
const VALUES_BIN_INV:[u16;9] = [
    0b1111111111111110,
    0b1111111111111101,
    0b1111111111111011,
    0b1111111111110111,
    0b1111111111101111,
    0b1111111111011111,
    0b1111111110111111,
    0b1111111101111111,
    0b1111111011111111,
];

const LEVEL:u8 = 50;

#[derive(Resource)]
pub struct Playfield {
    pub values: Array2D<u8>,
    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
    solved: bool,
}

fn p(cursor:usize, sign:&str) {
    let mut i = 0;
    while i < cursor {
        print!("{sign}");
        i += 1;
    }
    println!("{sign}");
}

impl Playfield {
    pub fn new() -> Self {
        Playfield {
            values: Array2D::filled_with(0, ROW_COUNT as usize, COL_COUNT as usize),
            poss_rows: [0b1111111111111111u16; 9],
            poss_cols: [0b1111111111111111u16; 9],
            poss_quads: [0b1111111111111111u16; 9],
            solved: false,
        }
    }

    pub fn set_value(&mut self, row:usize, col:usize, val:u8) {
        let quad = QUADS[row][col] as usize;
        let current_val = self.values[(row, col)];
        
        if val == current_val {
            return;
        }

        if val == 0 {
            let mov_zero_based = (current_val - 1) as usize;
            let mov_bin = VALUES_BIN[mov_zero_based];
            self.values[(row, col)] = 0;
            self.poss_rows[row] |= mov_bin;
            self.poss_cols[col] |= mov_bin;
            self.poss_quads[quad] |= mov_bin;
            return;
        }

        let mov_zero_based = (val - 1) as usize;
        let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
        self.poss_rows[row] &= mov_bin_inv;
        self.poss_cols[col] &= mov_bin_inv;
        self.poss_quads[quad] &= mov_bin_inv;
        self.values[(row, col)] = val;
    }

    fn generate_(&mut self, values: &mut Array2D<u8>, fields_queue: [usize; FIELDS_COUNT], cursor:usize, removed_count:u8) -> Option<Array2D<u8>> {
        if removed_count > LEVEL {
            if self.multiple_solutions_(values, cursor) > 1 {
                return Option::None;
            }
            return Option::Some(values.clone());
        }

        let field = FIELDS[fields_queue[cursor]];
        let row = field.0 as usize;
        let col = field.1 as usize;
        let quad = QUADS[row][col] as usize;

        let mov = values[(row, col)];
        let mov_zero_based = (mov - 1) as usize;
        let mov_bin = VALUES_BIN[mov_zero_based];
        let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];

        values[(row, col)] = 0;
        self.poss_rows[row] |= mov_bin;
        self.poss_cols[col] |= mov_bin;
        self.poss_quads[quad] |= mov_bin;

        // println!("{}{} delete field: {}", removed_count, "-".repeat(removed_count as usize), cursor);
        let solution = self.generate_(values, fields_queue, cursor + 1, removed_count + 1);
        if solution.is_some() {
            return solution;
        }

        // println!("{}{} reset field: {}", removed_count, "-".repeat(removed_count as usize), cursor);
        self.poss_rows[row] &= mov_bin_inv;
        self.poss_cols[col] &= mov_bin_inv;
        self.poss_quads[quad] &= mov_bin_inv;
        values[(row, col)] = mov;
        return self.generate_(values, fields_queue, cursor + 1, removed_count);
    }

    fn find_solution_(&mut self, values: &mut Array2D<u8>, cursor:usize) -> Option<Array2D<u8>> {
        if cursor >= 81 {
            return Option::Some(values.clone());
        }
        let field = FIELDS[cursor];
        let row = field.0 as usize;
        let col = field.1 as usize;
        let quad = QUADS[row][col] as usize;

        let field_val = values[(row,col)];
        if field_val > 0 {
            let option = self.find_solution_(values, cursor + 1);
            if option.is_some() {
                return option;     
            }
        } else {
            let poss = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
            for mov_zero_based in poss.view_bits::<Lsb0>()[0..9].iter_ones() {
                let mov_bin = VALUES_BIN[mov_zero_based];
                let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
    
                self.poss_rows[row] &= mov_bin_inv;
                self.poss_cols[col] &= mov_bin_inv;
                self.poss_quads[quad] &= mov_bin_inv;
                values[(row, col)] = mov_zero_based as u8 + 1;

                let option = self.find_solution_(values, cursor + 1);
                if option.is_some() {
                    return option;     
                }

                values[(row, col)] = 0;
                self.poss_rows[row] |= mov_bin;
                self.poss_cols[col] |= mov_bin;
                self.poss_quads[quad] |= mov_bin;
            }
        }

        return Option::None;
    }

    pub fn solve(&mut self) {
        let solution = self.find_solution_(&mut self.values.clone(), 0)
                .expect("No solution found");
        self.values = solution;
        self.solved = true;
    }

    pub fn generate(&mut self) {
        let mut values_random_mask: [usize; 9] = core::array::from_fn(|i| i + 1);
        values_random_mask.shuffle(&mut thread_rng());

        let mut cursor_random_mask: [usize; 81] = [0; 81];
        for i in 0..81 {
            cursor_random_mask[i] = i;
        }
        cursor_random_mask.shuffle(&mut thread_rng());

        let mut solution = self.find_solution_(&mut self.values.clone(), 0)
                .expect("No solution found");
        
        let generated = self.generate_(&mut solution, cursor_random_mask, 0, 0)
                .expect("No solution generated");

        self.values = generated;
    }

    fn multiple_solutions_(&mut self, values: &mut Array2D<u8>, cursor:usize) -> u8 {
        if cursor >= 81 {
            return 1;
        }
        let field = FIELDS[cursor];
        let row = field.0 as usize;
        let col = field.1 as usize;

        let field_val = values[(row,col)];

        let mut solution_count:u8 = 0;
        if field_val > 0 {
            solution_count += self.multiple_solutions_(values, cursor + 1);
        } else {
            let quad = QUADS[row][col] as usize;
            let poss: u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
            for mov_zero_based in poss.view_bits::<Lsb0>()[0..9].iter_ones() {
                let mov_bin = VALUES_BIN[mov_zero_based];
                let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
    
                self.poss_rows[row] &= mov_bin_inv;
                self.poss_cols[col] &= mov_bin_inv;
                self.poss_quads[quad] &= mov_bin_inv;
                values[(row, col)] = mov_zero_based as u8 + 1;
    
                solution_count += self.multiple_solutions_(values, cursor + 1);

                values[(row, col)] = 0;
                self.poss_rows[row] |= mov_bin;
                self.poss_cols[col] |= mov_bin;
                self.poss_quads[quad] |= mov_bin;

                if solution_count > 1 {
                    return 2;
                }
            }
        }
        
        return solution_count;
    }

    pub fn count_solutions(&mut self) -> u8 {
        self.multiple_solutions_(&mut self.values.clone(), 0)
    }

    // fn count_solutions_(&mut self, values: &mut Array2D<u8>, cursor:usize) -> u32 {
    //     if cursor >= 81 {
    //         // println!("solution found");
    //         return 1;
    //     }
    //     let field = FIELDS[cursor];
    //     let row = field.0 as usize;
    //     let col = field.1 as usize;

    //     let field_val = values[(row,col)];

    //     let mut solution_count = 0;
    //     if field_val > 0 {
    //         solution_count += self.count_solutions_(values, cursor + 1);
    //     } else {
    //         let quad = QUADS[row][col] as usize;
    //         let poss: u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
    //         for mov_zero_based in poss.view_bits::<Lsb0>()[0..9].iter_ones() {
    //             let mov_bin = VALUES_BIN[mov_zero_based];
    //             let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
    
    //             self.poss_rows[row] &= mov_bin_inv;
    //             self.poss_cols[col] &= mov_bin_inv;
    //             self.poss_quads[quad] &= mov_bin_inv;
    //             values[(row, col)] = mov_zero_based as u8 + 1;
    
    //             solution_count += self.count_solutions_(values, cursor + 1);
    
    //             values[(row, col)] = 0;
    //             self.poss_rows[row] |= mov_bin;
    //             self.poss_cols[col] |= mov_bin;
    //             self.poss_quads[quad] |= mov_bin;
    //         }
    //     }
    //     return solution_count;
    // }

    // fn count_solutions(&mut self) -> u32 {
    //     self.count_solutions_(&mut self.values.clone(), 0)
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_solution_counter_empty() {
//         let mut playfield = Playfield::new();
//         assert!(playfield.multiple_solutions());
//     }

//     #[test]
//     fn test_solution_counter_full() {
//         let mut playfield = Playfield::new();
//         playfield.solve();
//         assert!(!playfield.multiple_solutions());
//         // assert_eq!(1, playfield.count_solutions());
//     }

//     #[test]
//     fn test_solution_counter_partial() {
//         let mut playfield = Playfield::new();
//         playfield.solve();

//         let mut cursor_random_mask: [usize; 81] = [0; 81];
//         for i in 0..81 {
//             cursor_random_mask[i] = i;
//         }
//         cursor_random_mask.shuffle(&mut thread_rng());

//         for i in 0..20 {
//             let field = FIELDS[cursor_random_mask[i]];
//             playfield.set_value(field.0 as usize, field.1 as usize, 0);
//         }

//         // assert_eq!(1, playfield.count_solutions());
//         assert!(!playfield.multiple_solutions());
//     }

//     #[test]
//     fn test_solution_counter_partial_2() {
//         let mut playfield = Playfield::new();
//         playfield.set_value(0, 6, 7);
//         playfield.set_value(0, 8, 9);
//         playfield.set_value(1, 6, 1);
//         playfield.set_value(2, 3, 1);
//         playfield.set_value(2, 4, 2);
//         playfield.set_value(3, 4, 6);
//         playfield.set_value(4, 2, 5);
//         playfield.set_value(4, 3, 8);
//         playfield.set_value(4, 6, 2);
//         playfield.set_value(4, 8, 4);
//         playfield.set_value(5, 1, 9);
//         playfield.set_value(5, 2, 7);
//         playfield.set_value(5, 3, 2);
//         playfield.set_value(5, 7, 6);
//         playfield.set_value(5, 8, 5);
//         playfield.set_value(6, 0, 5);
//         playfield.set_value(6, 2, 1);
//         playfield.set_value(6, 5, 2);

//         assert!(playfield.multiple_solutions());
//     }

//     #[test]
//     fn test_solution_counter_partial_3() {
//         let mut playfield = Playfield::new();
//         playfield.set_value(0, 2, 3);
//         playfield.set_value(0, 4, 5);
//         playfield.set_value(0, 5, 6);
//         playfield.set_value(0,7,8);
//         playfield.set_value(0, 8,9);
//         playfield.set_value(1,0,4);
//         playfield.set_value(1,3,7);
//         playfield.set_value(1,4,8);
//         playfield.set_value(1,5,9);
//         playfield.set_value(1,6,1);
//         playfield.set_value(1,7,2);
//         playfield.set_value(2,2,9);
//         playfield.set_value(2,6,4);
//         playfield.set_value(2,8,6);
//         playfield.set_value(3,7,9);
//         playfield.set_value(4,0,3);
//         playfield.set_value(4,1,6);
//         playfield.set_value(4,5,7);
//         playfield.set_value(4,6,2);
//         playfield.set_value(5,2,7);
//         playfield.set_value(5,3,2);
//         playfield.set_value(5,4,1);
//         playfield.set_value(5,6,3);
//         playfield.set_value(5,7,6);
//         playfield.set_value(6,1,3);
//         playfield.set_value(6,4,4);
//         playfield.set_value(6,5,2);
//         playfield.set_value(6,8,8);
//         playfield.set_value(7,8,1);
//         playfield.set_value(8,8,2);
        
//         assert!(!playfield.multiple_solutions());
//     }
// }

#[derive(Resource)]
pub struct Status {
    pub text: String,
}