use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest  = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    //return largest without dereferencing
    largest.clone()
}

pub fn main_load() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_numer(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let char_genereic_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_genereic_list);
    println!("The largest char_genereic_list is {}", result);
}

fn largest_numer(vector: &Vec<i32>) -> i32 {
    let mut largest = vector[0];

    for &item in vector.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//convert largest_numer to generic function
fn largest_numer_generic<T: PartialOrd>(vector: &Vec<T>) -> &T {
    let mut largest = &vector[0];

    for item in vector.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}