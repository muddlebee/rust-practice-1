use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
#[derive(Debug)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn test(){
    let mut minheap = BinaryHeap::new();
    minheap.push(MinNonNan(2.0));
    minheap.push(MinNonNan(1.0));
    minheap.push(MinNonNan(42.0));

    //loop till minheap is empty
    while !minheap.is_empty() {
        let min = minheap.pop().unwrap();
        println!("min = {:?}", min);
    }
}