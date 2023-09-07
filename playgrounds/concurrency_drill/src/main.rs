use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::windows::prelude::HandleOrInvalid;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file_mutex = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .write(true)
            .create(true)
            .open("increments.txt")
            .unwrap(),
    ));

    let mut handles = vec![];

    for i in 0..10 {
        let file_mutex = Arc::clone(&file_mutex);
        let handle = thread::spawn(move || {
            let mut file = file_mutex.lock().unwrap();
            writeln!(file, "{}", i).unwrap();
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
