use jwalk::WalkDir;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start = Instant::now();
    let count = run_jwalk(&args[1]);
    let end = Instant::now();
    println!("The time {:?}, and the files {:?}", end.duration_since(start), count);
}

fn run_jwalk(directory: &String) -> u32 {
    let mut count = 0;
    for entry in WalkDir::new(directory).sort(true) {
        count += 1;
    }
    count
}
