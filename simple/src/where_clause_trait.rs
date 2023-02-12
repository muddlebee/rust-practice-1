use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}


// implement PrintInOption with Ord trait and Debug trait
impl<T> PrintInOption for T where
    T: Ord + Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
