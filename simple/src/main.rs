mod collections;

// guess game from TRLB
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    collections::count_word();
}
