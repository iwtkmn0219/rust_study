use ::rand::Rng;

fn main() {
    // Create random number creator
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}
