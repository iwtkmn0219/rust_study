use ::std::fs::File;
use ::std::io::{BufRead, BufReader};

fn main() {
    // dictionary file
    let dictionary_file = "data/dict.txt";
    // allocate args
    let args: Vec<String> = std::env::args().collect();
    // check args
    if args.len() < 2 {
        println!("Please input args");
        return;
    }
    // word
    let word = &args[1];
    // open file
    let fp = File::open(dictionary_file).unwrap();
    // read bufreader
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.find(word) == None {
            continue;
        }
        println!("{}", line);
    }
}
