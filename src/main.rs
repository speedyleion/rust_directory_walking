use jwalk::WalkDir;
use std::{env, fs};
use std::time::Instant;

use directory_walking::ntquerydirectoryfile::ntquery_walk_dir;
use directory_walking::read_dir::walk_dir_threaded;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    // Run once to warm up file cache
    run_jwalk(&path);

    let start = Instant::now();
    let count = run_jwalk(&path);
    let end = Instant::now();
    println!("The jwalk time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = ntquery_walk_dir(&path);
    let end = Instant::now();
    println!("The ntquery_walk_dir time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = walk_dir_path_is_dir(&path);
    let end = Instant::now();
    println!("The walk_dir_path_is_dir time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = walk_dir(&path);
    let end = Instant::now();
    println!("The walk_dir time {:?}, and the files {:?}", end.duration_since(start), count);

    let start = Instant::now();
    let count = walk_dir_threaded(&path);
    let end = Instant::now();
    println!("The walk_dir_threaded time {:?}, and the files {:?}", end.duration_since(start), count);
}

fn run_jwalk(path: &Path) -> usize {
    WalkDir::new(path).sort(true).into_iter().count()
}

fn walk_dir(dir: &Path) -> usize {
    let mut count = 0;
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            count += walk_dir(&path);
        } else {
            files.push(entry);
        }
    }

    count += files.len();
    count
}

fn walk_dir_path_is_dir(dir: &Path) -> usize {
    let mut count = 0;
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            count += walk_dir(&path);
        } else {
            files.push(entry);
        }
    }

    count += files.len();
    count
}
