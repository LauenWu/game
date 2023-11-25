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

struct FieldIterator {
    fields: [(u8, u8); FIELDS_COUNT as usize],
    current_index: usize,
}

impl FieldIterator {
    fn new() -> Self {
        let mut fields = FIELDS.clone();
        //fields.shuffle(&mut thread_rng());
        FieldIterator {fields, current_index: 0,}
    }
}

impl Iterator for FieldIterator {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.fields.len() > self.current_index  {
            let number = self.fields[self.current_index];
            self.current_index += 1;
            return Some(number);
        } else {
            return None;
        }
    }
}

pub fn solve(playfield: &mut Playfield) {
    playfield.solve();
}

trait Solvable {
    fn get_cursor(&self) -> u8;
    fn possible_moves_at(&self, field: (u8, u8)) -> Vec<u8>;
    fn make_move_at(&mut self, field: (u8, u8), mov:u8, values: &mut Array2D<u8>);
    fn revert_move_at(&mut self, field: (u8, u8), mov:u8, values: &mut Array2D<u8>);
    fn check(&mut self, values: &Array2D<u8>);
    fn is_solved(&self) -> bool;
    fn solve(&mut self);
    fn get_field(&self, cursor: u8) -> (u8, u8);
    fn inc_cursor(&mut self);
    fn dec_cursor(&mut self);

    fn solve_(&mut self, values: &mut Array2D<u8>) {
        println!("in");
        let cursor = self.get_cursor();
        if cursor < 81 {
            let field = self.get_field(cursor);
            let mut possible_moves = self.possible_moves_at(field);
            println!("{:?}, {:?}", cursor, possible_moves);

            //possible_moves.shuffle(&mut thread_rng());

            for possible_move in possible_moves {
                self.make_move_at(field, possible_move, values);
                self.inc_cursor();
                self.solve_(values);
                if self.is_solved() {
                    return;
                }
                self.dec_cursor();
                self.revert_move_at(field, possible_move, values);
            }
        } else {
            // field iterator has no next           
            self.check(values);
        }
        println!("ret")
    }
}

#[derive(Resource)]
pub struct Playfield {
    pub values: Array2D<u8>,
    blocked_rows: [BitArray<[u16; 1], Msb0>; 9],
    blocked_cols: [BitArray<[u16; 1], Msb0>; 9],
    blocked_quads: [BitArray<[u16; 1], Msb0>; 9],
    solved: bool,
    cursor: u8,
    fields: [(u8, u8); FIELDS_COUNT as usize],
}

impl Playfield {
    pub fn new() -> Self {
        let arr = bitarr!(u16, Msb0; 0; 9);
        let b: [BitArray<[u16; 1], Msb0>; 9] = [
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),
            arr.clone(),            
        ];

        let mut fields = FIELDS.clone();
        //fields.shuffle(&mut thread_rng());

        Playfield {
            values: Array2D::filled_with(0, ROW_COUNT as usize, COL_COUNT as usize),
            blocked_rows: b.clone(),
            blocked_cols: b.clone(),
            blocked_quads: b.clone(),
            solved: false,
            cursor: 0,
            fields,
        }
    }
}

impl Solvable for Playfield {
    fn possible_moves_at(&self, field: (u8, u8)) -> Vec<u8> {
        let (row, col) = field;
        let blocked = self.blocked_rows[row as usize]
            .bitor(self.blocked_cols[col as usize])
            .bitor(self.blocked_quads[QUADS[row as usize][col as usize] as usize]);
        blocked[0..9].iter_zeros().map(|a| (a + 1) as u8).collect::<Vec<u8>>()
    }

    fn make_move_at(&mut self, field: (u8, u8), mov:u8, values: &mut Array2D<u8>) {
        let row = field.0 as usize;
        let col = field.1 as usize;
        self.blocked_rows[row].set((mov-1) as usize, true);
        self.blocked_cols[col].set((mov-1) as usize, true);
        self.blocked_quads[QUADS[row][col] as usize].set((mov-1) as usize, true);
        values[(row, col)] = mov;
    }

    fn revert_move_at(&mut self, field: (u8, u8), mov:u8, values: &mut Array2D<u8>) {
        let row = field.0 as usize;
        let col = field.1 as usize;
        self.blocked_rows[row as usize].set((mov-1) as usize, false);
        self.blocked_cols[col as usize].set((mov-1) as usize, false);
        self.blocked_quads[QUADS[row as usize][col as usize] as usize].set((mov-1) as usize, false);
        values[(row, col)] = 0;
    }

    fn check(&mut self, values: &Array2D<u8>) {
        let mut solved = true;
        for row in values.rows_iter() {
            for field in row {
                if *field == 0 {
                    solved = false;
                }
            }
        }
        self.values = values.clone();
        self.solved = solved;

        if solved {
            for row in 0..9 {
                for col in 0..9 {
                    print!("{:?} ", values[(row, col)]);
                }
                println!("");
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.solved
    }

    fn solve(&mut self) {
        self.solve_(&mut self.values.clone());
    }

    fn get_cursor(&self) -> u8 {
        self.cursor
    }

    fn get_field(&self, cursor: u8) -> (u8, u8) {
        self.fields[cursor as usize]
    }

    fn inc_cursor(&mut self) {
        self.cursor += 1;
    }

    fn dec_cursor(&mut self) {
        self.cursor -= 1;
    }
}

#[derive(Resource)]
pub struct Status {
    pub text: String,
}