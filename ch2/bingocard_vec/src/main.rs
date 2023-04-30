use ::rand::seq::SliceRandom;

fn main() {
    // Create vector 1~75
    let mut nums = Vec::new();
    for i in 1..=75 {
        nums.push(i);
    }
    // Random number creator
    let mut rng = rand::thread_rng();
    // Shuffle array
    nums.shuffle(&mut rng);

    // Show array
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!(" *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!();
    }
}
