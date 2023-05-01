use ::std::env; // read env
use ::std::fs; // read file

fn main() {
    // get args in execute line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please input file name");
        return;
    }
    let filename = &args[1];
    println!("{}", filename);
    // read file
    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };
    println!("{}", text);
}
