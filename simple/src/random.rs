use rand::Rng;

pub fn random_range() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(90, 101);
    println!("random number is {}", n);
}