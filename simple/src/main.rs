mod collections;
mod generics_trait;
mod struct_example;
mod random;
mod string;
mod OrdStruct;
mod binary_heap;
mod shared_ownership;
mod where_clause_trait;
mod option_utils;
mod generic_larger_object;
mod result;

// guess game from TRLB
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::{io, vec};

pub type logType = Vec<u32>;


fn main() {
    generic_larger_object::main_load();
}
