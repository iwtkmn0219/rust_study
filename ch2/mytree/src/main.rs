use ::std::{env, fs, path};

static INIT: usize = 0;

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    // if no args, target is present directory
    let mut target_directory: &str = ".";
    if args.len() >= 2 {
        target_directory = &args[1];
    }
    // get PathBuf
    let target = path::PathBuf::from(target_directory);
    // get absolute path
    let absolute_path = fs::canonicalize(target_directory).unwrap();
    println!("{:?}", absolute_path);
    tree(target, INIT);
}

fn tree(target: path::PathBuf, level: usize) {
    // get file list
    let files = target.read_dir().expect("this path is unvailable");

    for file in files {
        // get PathBuf
        let path = file.unwrap().path();
        // indent as level
        for _ in 0..level {
            print!("│   ");
        }
        // get file name
        let file_name = path.file_name().unwrap().to_string_lossy();
        // if directory show recursive
        if path.is_dir() {
            println!("├──< {} >", file_name);
            tree(path, level + 1);
        } else {
            println!("├── {}", file_name);
        }
    }
}
