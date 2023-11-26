use bevy::prelude::*;
use array2d::Array2D;
use bitvec::prelude::*;
use rand::prelude::*;
use std::ops::BitAndAssign;
use std::ops::BitOr;

const ROW_COUNT:u8 = 9;
const COL_COUNT:u8 = 9;
const QUAD_COUNT:u8=9;
const FIELDS_COUNT:u8=81;
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
const VALUES:[u8;9] = [1,2,3,4,5,6,7,8,9];

pub fn solve(playfield: &mut Playfield) {
    playfield.solve();
}

#[derive(Resource)]
pub struct Playfield {
    pub values: Array2D<u8>,
    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
    cursor: usize,
    fields: [(u8, u8); FIELDS_COUNT as usize],
    values_map: [u16; 9],
    value_random_mask: [u8; 9],
}

impl Playfield {
    pub fn new() -> Self {
        let mut fields = FIELDS.clone();
        //fields.shuffle(&mut thread_rng());

        let mut value_random_mask = VALUES.clone();
        value_random_mask.shuffle(&mut thread_rng());
        // maps from 00010 to 2
        // let mut digit_map: [u8; (1<<8)+1] = [0; (1<<8)+1];

        // maps from 0 to 00001
        let mut values_map: [u16; 9] = [0; 9];

        for i in 0..9 {
            let ii: u16 = 1 << i;
            // i:0 -> ii:1
            // digit_map[ii as usize] = i + 1;
            values_map[i as usize] = ii; 
        }

        Playfield {
            values: Array2D::filled_with(0, ROW_COUNT as usize, COL_COUNT as usize),
            poss_rows: [511u16; 9],
            poss_cols: [511u16; 9],
            poss_quads: [511u16; 9],
            cursor: 0,
            fields,
            values_map,
            value_random_mask,
        }
    }

    fn solve_(&mut self, values: &mut Array2D<u8>) -> bool {
        if self.cursor >= 81 {
            for row in 0..9 {
                for col in 0..9 {
                    self.values[(row, col)] = self.value_random_mask[(values[(row, col)] - 1) as usize]
                }
            }
            //self.values = values.clone();
            return true;
        }
        let cursor = self.cursor;
        let field = self.fields[cursor];
        let row = field.0 as usize;
        let col = field.1 as usize;
        let quad = QUADS[row][col] as usize;

        let poss: u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];

        for mov_zero_based in poss.view_bits::<Lsb0>().iter_ones() {
            let mov_bin = self.values_map[mov_zero_based];
            let mov_bin_inv = !mov_bin;

            self.poss_rows[row] &= mov_bin_inv;
            self.poss_cols[col] &= mov_bin_inv;
            self.poss_quads[quad] &= mov_bin_inv;
            values[(row, col)] = mov_zero_based as u8 + 1;
            self.cursor += 1;

            if self.solve_(values) {
                return true;
            }

            self.cursor -= 1;
            values[(row, col)] = 0;
            self.poss_rows[row] |= mov_bin;
            self.poss_cols[col] |= mov_bin;
            self.poss_quads[quad] |= mov_bin;
        }
        return false;
    }

    fn solve(&mut self) {
        self.solve_(&mut self.values.clone());
    }
}

#[derive(Resource)]
pub struct Status {
    pub text: String,
}