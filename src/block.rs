use std::collections::HashMap;

use super::BFToken;

struct BasicBlock {
    does_io: bool,
    offset: i64,
    instructions: Vec<Instruction>,
}

enum Instruction {
    CallRead { offset: i64 },
    CallWrite { offset: i64 },
    Mul,
    Add,
    Sub,
    Mov,
}

#[derive(PartialEq)]
enum CellState {
    ReadModified(u64, i64),
    Original(i64),
}

impl CellState {
    fn inc(&mut self) {
        match self {
            CellState::ReadModified(_, m) => *m += 1,
            CellState::Original(m) => *m += 1,
        }
    }
    fn dec(&mut self) {
        match self {
            CellState::ReadModified(_, m) => *m -= 1,
            CellState::Original(m) => *m -= 1,
        }
    }
}

impl BasicBlock {
    fn new(v: &[BFToken]) -> Self {
        let mut changes = HashMap::new();
        let mut offset = 0;
        let mut read_number = 0;

        for token in v {
            match token {
                BFToken::Left => offset -= 1,
                BFToken::Right => offset += 1,
                BFToken::Plus => changes
                    .entry(offset)
                    .or_insert(CellState::Original(0))
                    .inc(),
                BFToken::Minus => changes
                    .entry(offset)
                    .or_insert(CellState::Original(0))
                    .dec(),
                BFToken::Write => (),
                BFToken::Read => {
                    changes.insert(offset, CellState::ReadModified(read_number, 0));
                    read_number += 1;
                }
            }
        }

        changes.retain(|_, change| *change != CellState::Original(0));

        todo!()
    }
}
