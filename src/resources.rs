use bevy::prelude::*;
use array2d::Array2D;
use bitvec::prelude::*;
use rand::prelude::*;

const ROW_COUNT:u8 = 9;
const COL_COUNT:u8 = 9;
const FIELDS_COUNT:u8 = 81;
const FIELDS:[(u8, u8); FIELDS_COUNT as usize] = [
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

const LEVEL:u8 = 5;

pub fn solve(playfield: &mut Playfield) {
    playfield.solve();
}

pub fn generate(playfield: &mut Playfield) {
    playfield.generate();
}

#[derive(Resource)]
pub struct Playfield {
    pub values: Array2D<u8>,
    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
    empty_fields: u8,
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
            empty_fields: 81,
            solved: false,
        }
    }

    fn generate_(&mut self, values: &mut Array2D<u8>, cursor:usize, solved:bool, cursor_random_mask:[usize;81]) -> Option<Array2D<u8>> {
        if solved {
            p(cursor, "-");
            if self.empty_fields > LEVEL {
                println!("generated solution found");
                return Option::Some(values.clone());
            }

            if cursor >= 81 {
                println!("no solution generated");
                return Option::None;
            }

            let field = FIELDS[cursor_random_mask[cursor]];
            let row = field.0 as usize;
            let col = field.1 as usize;
            let quad = QUADS[row][col] as usize;

            let mov = values[(row, col)];
            let mov_zero_based = (mov - 1) as usize;
            let mov_bin = VALUES_BIN[mov_zero_based];
            let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];

            self.empty_fields += 1;
            values[(row, col)] = 0;
            self.poss_rows[row] |= mov_bin;
            self.poss_cols[col] |= mov_bin;
            self.poss_quads[quad] |= mov_bin;

            let poss = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
            if poss.view_bits::<Lsb0>()[0..9].count_ones() == 1 {
                let solution = self.generate_(values, cursor + 1, true, cursor_random_mask);
                if solution.is_some() {
                    return solution;
                }
            }

            self.poss_rows[row] &= mov_bin_inv;
            self.poss_cols[col] &= mov_bin_inv;
            self.poss_quads[quad] &= mov_bin_inv;
            values[(row, col)] = mov;
            self.empty_fields -= 1;

            let solution = self.generate_(values, cursor + 1, true, cursor_random_mask);
            if solution.is_some() {
                return solution;
            }

            return Option::None;
        } else {
            p(cursor, "x");
            if cursor >= 81 {
                println!("solved");
                return self.generate_(values, 0, true, cursor_random_mask);
            }
            
            let field = FIELDS[cursor];
            let row = field.0 as usize;
            let col = field.1 as usize;
            let quad = QUADS[row][col] as usize;

            let field_val = values[(row,col)];
            if field_val > 0 {
                let option = self.generate_(values, cursor + 1, false, cursor_random_mask);
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
                    self.empty_fields -= 1;
        
                    let option = self.generate_(values, cursor + 1, false, cursor_random_mask);
                    if option.is_some() {
                        return option;       
                    }
        
                    self.empty_fields += 1;
                    values[(row, col)] = 0;
                    self.poss_rows[row] |= mov_bin;
                    self.poss_cols[col] |= mov_bin;
                    self.poss_quads[quad] |= mov_bin;
                }
            }

            return Option::None;
        }
    }

    fn find_solution(&mut self, values: &mut Array2D<u8>, cursor:usize) -> Option<Array2D<u8>> {
        p(cursor, "s");
        if cursor >= 81 {
            return Option::Some(values.clone());
        }
        let field = FIELDS[cursor];
        let row = field.0 as usize;
        let col = field.1 as usize;
        let quad = QUADS[row][col] as usize;

        let field_val = values[(row,col)];
        if field_val > 0 {
            let option = self.find_solution(values, cursor + 1);
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
                self.empty_fields -= 1;
    
                let option = self.find_solution(values, cursor + 1);
                if option.is_some() {
                    return option;     
                }
    
                self.empty_fields += 1;
                values[(row, col)] = 0;
                self.poss_rows[row] |= mov_bin;
                self.poss_cols[col] |= mov_bin;
                self.poss_quads[quad] |= mov_bin;
            }
        }

        
        return Option::None;
    }

    fn solve(&mut self) {
        let solution = self.find_solution(&mut self.values.clone(), 0)
                .expect("No solution found");
        self.values = solution;
        self.solved = true;
    }

    fn generate(&mut self) {
        let mut values_random_mask: [usize; 9] = core::array::from_fn(|i| i + 1);
        values_random_mask.shuffle(&mut thread_rng());

        let mut cursor_random_mask: [usize; 81] = [0; 81];
        let mut c = 0;
        for i in cursor_random_mask.iter_mut() {
            *i = c;
            c += 1;
        }
        cursor_random_mask.shuffle(&mut thread_rng());

        let solution = self.generate_(&mut self.values.clone(), 0, false, cursor_random_mask)
                .expect("No solution generated");

        self.values = solution;
    }

    // fn unique_solution(&mut self, values: &mut Array2D<u8>, cursor:usize, sol_count:&mut u8) -> u8 {
    //     if cursor >= 81 {
    //         return *sol_count + 1;
    //     }
    //     let field = FIELDS[cursor];
    //     let row = field.0 as usize;
    //     let col = field.1 as usize;
    //     let quad = QUADS[row][col] as usize;

    //     let poss: u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];

    //     for mov_zero_based in poss.view_bits::<Lsb0>()[0..9].iter_ones() {
    //         let mov_bin = VALUES_BIN[mov_zero_based];
    //         let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];

    //         self.poss_rows[row] &= mov_bin_inv;
    //         self.poss_cols[col] &= mov_bin_inv;
    //         self.poss_quads[quad] &= mov_bin_inv;
    //         values[(row, col)] = mov_zero_based as u8 + 1;
    //         self.empty_fields -= 1;

    //         *sol_count += self.unique_solution(values, cursor + 1, sol_count);
    //         if *sol_count > 1 {
    //             return *sol_count;
    //         }

    //         self.empty_fields += 1;
    //         values[(row, col)] = 0;
    //         self.poss_rows[row] |= mov_bin;
    //         self.poss_cols[col] |= mov_bin;
    //         self.poss_quads[quad] |= mov_bin;
    //     }
    //     return 0;
    // }
}

#[derive(Resource)]
pub struct Status {
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // TODO
    }
}