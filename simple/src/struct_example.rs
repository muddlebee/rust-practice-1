pub fn example(){
    struct Person {
        age: u8,
        is_child: bool,
        log: Vec<u32>

    }
    struct OnePerson(u8, bool);
    struct UnitStruct;
    let alice = Person {age: 10, is_child: true, log: Vec::new()};
    let bob = OnePerson(32, false);
    let x = UnitStruct;
    println!("alice age {} is child {}", alice.age, alice.is_child);
    println!("bob age {} is child {}", bob.0, bob.1);
    println!("unit struct {:p}", &x);

    impl Person {
        fn create_person(age: u8, is_child: bool, log: Vec<u32>) -> Person {
            Person{age, is_child, log}
        }
        fn check_child(&self) -> bool {
            if self.is_child && self.age < 18 {
                return true;
            } else {
                return false;
            }
        }

        fn set_age(&mut self, age: u8) {
            self.age = age;
        }

        fn increment_age(&mut self) {
            self.age += 1;
        }

        fn append_log(&mut self, log: u32) {
            self.log.push(log);
        }

        fn delete_log_index(&mut self, index: usize) {
            self.log.remove(index);
        }

        fn delete_log(&mut self, log: u32) {
            let mut index = 0;
            for i in 0..self.log.len() {
                if self.log[i] == log {
                    index = i;
                    break;
                }
            }
            self.log.remove(index);
        }
    }
    let mut peter = Person::create_person(33, true, Vec::new());
    println!("peter age {} is child {}", peter.age, peter.is_child);
    println!("peter is child {}", peter.check_child());

    peter.increment_age();
    println!("peter age {} is child {}", peter.age, peter.is_child);

    peter.append_log(1);
    peter.append_log(2);
    println!("peter log {:?}", peter.log);

    peter.delete_log_index(1);
    println!("peter log {:?}", peter.log);
}