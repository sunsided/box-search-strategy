//! # Box searching simulation.
//!
//! Eight boxes are filled with two coins, where all boxes have equal
//! probability to have a coin. Alice searches the boxes left to right,
//! then row by row, whereas Bob searches the boxes top to bottom,
//! column by column. The first person to find a coin wins.
//! Are Alice and Bob equally likely to win?
//!
//! ```text
//!  ________________
//! |   |   | x |   |
//! -----------------
//! |   | x |   |   |
//! -----------------
//! ```

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use rand::{thread_rng, Rng};

fn main() {
    // Default experiment (wider than high, two coins)
    let experiment = Experiment::default();
    experiment.run();

    // Higher than wide (two coins).
    let experiment = Experiment::new(100_000, 8, 2, 2);
    experiment.run();

    // Wide square (two coins).
    let experiment = Experiment::new(100_000, 8, 8, 2);
    experiment.run();

    // Wide square (four coins).
    let experiment = Experiment::new(100_000, 8, 8, 4);
    experiment.run();
}

struct Experiment {
    trials: usize,
    rows: usize,
    columns: usize,
    coins: usize
}

impl Default for Experiment {
    fn default() -> Self {
        Self::new(100_000, 2, 8, 2)
    }
}

impl Experiment {
    pub fn new(trials: usize, rows: usize, columns: usize, coins: usize) -> Self {
        Self {trials, rows, coins, columns }
    }

    pub fn run(&self) {
        let mut alice_wins = 0;
        let mut bob_wins = 0;

        for _ in 0..self.trials {
            let boxes = Stack::new(self.rows, self.columns, self.coins);

            // Run both search strategies.
            let alice = boxes.iter_rowwise().position(|b| b.is_coin()).unwrap_or(usize::MAX);
            let bob = boxes.iter_colwise().position(|b| b.is_coin()).unwrap_or(usize::MAX);

            if alice < bob {
                alice_wins += 1;
            } else if bob < alice {
                bob_wins += 1;
            }
        }

        let ties = self.trials - (alice_wins + bob_wins);

        println!("Outcome after for {} rows, {} columns, {} coins ({} trials)", self.rows, self.columns, self.coins, self.trials);
        println!("  row-wise wins:    {alice_wins} ({:.2}%)", 100.0 * alice_wins as f64 / (alice_wins as f64 + bob_wins as f64));
        println!("  column-wise wins: {bob_wins} ({:.2}%)", 100.0 * bob_wins as f64 / (alice_wins as f64 + bob_wins as f64));
        println!("  ties:             {ties} ({:.2}%)", 100.0 * ties as f64 / (alice_wins as f64 + bob_wins as f64));
    }
}

/// A stack of boxes.
struct Stack {
    pub rows: usize,
    pub columns: usize,
    boxes: Vec<Box>,
}

/// A box.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Box {
    Empty,
    Coin
}

impl Stack {
    pub fn new(rows: usize, columns: usize, num_coins: usize) -> Stack {
        Self {
            rows,
            columns,
            boxes: Self::stack_boxes(rows, columns, num_coins),
        }
    }

    pub fn box_at(&self, row: usize, colum: usize) -> Box {
        assert!(row < self.rows);
        assert!(colum < self.columns);
        self.boxes[row * self.columns + colum]
    }

    pub fn iter_rowwise(&self) -> RowwiseIterator {
        RowwiseIterator { row: 0, column: 0, stack: self }
    }

    pub fn iter_colwise(&self) -> ColumnwiseIterator {
        ColumnwiseIterator { row: 0, column: 0, stack: self }
    }

    fn stack_boxes(rows: usize, columns: usize, mut num_coins: usize) -> Vec<Box> {
        assert_ne!(rows, 0, "must have at least one row");
        assert_ne!(columns, 0, "must have at least one column");

        let num_boxes = rows * columns;
        if num_coins == num_boxes {
            return vec![Box::Coin; num_boxes];
        }

        let mut boxes = vec![Box::Empty; num_boxes];
        if num_coins == 0 {
            return boxes;
        }

        while num_coins > 0 {
            let b: usize = thread_rng().gen_range(0..num_boxes);
            if boxes[b].is_coin() {
                continue;
            }

            boxes[b] = Box::Coin;
            num_coins -= 1;
        }

        boxes
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.columns {
                match self.boxes[r * self.columns + c] {
                    Box::Coin => write!(f, "X")?,
                    Box::Empty => write!(f, "O")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Box {
    pub fn is_coin(&self) -> bool {
        *self == Box::Coin
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BoxAt {
    row: usize,
    column: usize,
    b: Box
}

impl Deref for BoxAt {
    type Target = Box;

    fn deref(&self) -> &Self::Target {
        &self.b
    }
}

struct RowwiseIterator<'a> {
    stack: &'a Stack,
    row: usize,
    column: usize
}

struct ColumnwiseIterator<'a> {
    stack: &'a Stack,
    row: usize,
    column: usize
}

impl<'a> Iterator for RowwiseIterator<'a> {
    type Item = BoxAt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.stack.rows || self.column >= self.stack.columns {
            return None;
        }

        let b = self.stack.box_at(self.row, self.column);
        let b = BoxAt { row: self.row, column: self.column , b };

        self.column += 1;
        if self.column == self.stack.columns {
            self.column = 0;
            self.row += 1;
        }

        Some(b)
    }
}


impl<'a> Iterator for ColumnwiseIterator<'a> {
    type Item = BoxAt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.stack.rows || self.column >= self.stack.columns {
            return None;
        }

        let b = self.stack.box_at(self.row, self.column);
        let b = BoxAt { row: self.row, column: self.column , b };

        self.row += 1;
        if self.row == self.stack.rows {
            self.row = 0;
            self.column += 1;
        }

        Some(b)
    }
}