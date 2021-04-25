//          Copyright Nick G 2021.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::fs;

pub fn walk_dir_threaded(path: &Path) -> usize {
    let count = Arc::new(AtomicUsize::new(0));
    {
        rayon::scope(|s| {
            get_dir_stats(path, s, &Arc::clone(&count));
        });
    }
    count.load(Ordering::Relaxed)
}


fn get_dir_stats(path: &Path, scope: &rayon::Scope, count: &Arc<AtomicUsize>) {

    let mut files = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let dir_path = entry.path();
            let cloned_count = Arc::clone(count);
            scope.spawn(move |s| {
                get_dir_stats(&dir_path, s, &cloned_count);
            });
        } else {
            files.push(entry);
        }
    }
    count.fetch_add(files.len() + 1, Ordering::Relaxed);

}
