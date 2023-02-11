#![allow(unused)]
pub fn ord() {
    use std::cmp::Ordering;

    #[derive(Eq)]
    #[derive(Debug)]
    struct Person {
        id: u32,
        name: String,
        height: u32,
    }

    impl Ord for Person {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.height > other.height {
                Ordering::Greater
            } else if self.height < other.height {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.height == other.height
        }
    }

    fn new_person(id: u32, name: &str, height: u32) -> Person {
        Person {
            id,
            name: name.to_string(),
            height,
        }
    }

    let mut people = vec![
        new_person(1, "Alice", 170),
        new_person(2, "Bob", 180),
        new_person(3, "Charlie", 160),
        new_person(3, "Charlie2", 1620),

        new_person(3, "Charlie3", 1610),
        new_person(3, "Charlie4", 16110),

    ];

    let p1 = new_person(3, "Charlie4", 16110);
    let p2 = new_person(3, "Charlie4", 16210);
    println!("condition : {:?}", p1 < p2);

    //pop people till empty
    while !people.is_empty() {
        let person = people.pop().unwrap();
        println!("person = {:?}", person);
    }

}