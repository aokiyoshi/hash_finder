use clap::Parser;
use sha256::digest;
use std::{
    sync::{Arc, Mutex},
    thread,
};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of zeros
    #[arg(short, long)]
    number: usize,

    /// Number of results
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}


fn find_hash_with_n_zeroes_in_range(n: usize, start: u32, end: u32) -> Option<(u32, String)> {
    for num in start..end {
        let hash = digest(num.to_string());
        let tail = (0..n).map(|_| "0").collect::<String>();
        if hash.ends_with(&tail) {
            return Some((num, hash));
        }
    }
    None
}

fn find_hash_with_n_zeroes(n: usize, q: usize) -> Vec<(u32, String)> {
    // each new thread gets a range of numbers with len = step, so on, until we find the required number of matches

    // prepare some constants
    let step = 1000;
    let mut start = 1;

    // creating mutex with empty vec
    let mutex = Arc::new(Mutex::new(Vec::new()));

    loop {
        // cloning mutex
        let c_mutex = Arc::clone(&mutex);

        // check if we got enough nums
        if c_mutex.lock().unwrap().len() == q {
            break;
        }

        // spawn thread for searching hash
        thread::spawn(
            move || match find_hash_with_n_zeroes_in_range(n, start, start + step) {
                Some(x) => c_mutex.lock().unwrap().push(x),
                None => (),
            },
        )
        .join()
        .expect("thread::spawn failed");

        // change range start for next search
        start = start.saturating_add(step + 1);

        //break if reach max
        if start == u32::MAX {
            break;
        }
    }

    // cloning mutex again for return
    let res = mutex.lock().unwrap().clone();

    res
}

fn main() {
    let args = Args::parse();

    let n = args.number;
    let q = args.count;

    println!("Starting search {} hashes with {} zeroes ...", q, n);

    for res in find_hash_with_n_zeroes(n, q) {
        println!("{}, {:?}", res.0 as u32, res.1);
    }
}
