use std::fs::File;
use std::io::prelude::*;
use std::io;

struct Info {
    name: String,
    age: i32,
    rating: i32,
}
//
// fn write_info(info: &Info) -> io::Result<()> {
//     // Early return on error
//     let mut file = match File::create("my_best_friends.txt") {
//         Err(e) => return Err(e),
//         Ok(f) => f,
//     };
//     if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
//         return Err(e)
//     }
//     if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
//         return Err(e)
//     }
//     if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
//         return Err(e)
//     }
//     Ok(())
// }
//
//
// use std::fs::File;
// use std::io::prelude::*;
// use std::io;
//
// struct Info {
//     name: String,
//     age: i32,
//     rating: i32,
// }

fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt")?;
    // Early return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}

fn main() {
    let info = Info {
        name: "John Doe".to_string(),
        age: 32,
        rating: 10,
    };
    write_info(&info).unwrap();
}