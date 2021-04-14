use jwalk::WalkDir;
use std::env;
use std::time::Instant;

use directory_walking::ntquerydirectoryfile::dir_walk;

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
}

fn run_jwalk(directory: &str) -> usize {
    let mut count = 0;
    for _ in WalkDir::new(directory).sort(true) {
        count += 1;
    }
    count
}
