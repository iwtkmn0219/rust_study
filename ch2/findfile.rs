use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("fildefile (path) (keyword)");
        return;
    }
    // Get args
    let target_dir = &args[1];
    let keyword = &args[2];
    // Change to PathBuf
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    // Get file list
    let files = target.read_dir().expect("Not exists Path!");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        let fname = path.file_name().unwrap().to_string_lossy();
        if None == fname.find(keyword) {
            continue;
        }
        println!("{}", path.to_string_lossy());
    }
}
