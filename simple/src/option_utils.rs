
//rust pointer reference example
pub fn as_mut() {
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v += 42,
        None => {},
    }
    println!("{:?}", x);
}