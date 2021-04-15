use jwalk::WalkDir;
use std::{env, fs};
use std::time::Instant;

use directory_walking::ntquerydirectoryfile::dir_walk;
use directory_walking::read_dir::visit_dirs_threaded;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start = Instant::now();
    let count = run_jwalk(&args[1]);
    let end = Instant::now();
    println!("The jwalk time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = dir_walk(&args[1]);
    let end = Instant::now();
    println!("The ntquerydirectoryfile time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = visit_dirs(Path::new(&args[1]));
    let end = Instant::now();
    println!("The visit_dir time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = visit_dirs_threaded(Path::new(&args[1]));
    let end = Instant::now();
    println!("The visit_dirs_threaded time {:?}, and the files {:?}", end.duration_since(start), count);
}

fn run_jwalk(directory: &str) -> usize {
    let mut count = 0;
    for _ in WalkDir::new(directory).sort(true) {
        count += 1;
    }
    count
}

fn visit_dirs(dir: &Path) -> usize {
    let mut count = 0;
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            count += visit_dirs(&path);
        } else {
            files.push(entry);
        }
    }

    count += files.len();
    count
}
