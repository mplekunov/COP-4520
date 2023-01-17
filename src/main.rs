use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Instant, io::Write,
};

fn main() {
    let start = Instant::now();

    let counter = Arc::new(Mutex::new(0));

    let prime_vector: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let mut thread_vector = Vec::new();

    for _ in 0..8 {
        let counter_copy = counter.clone();
        let prime_vectory_copy = prime_vector.clone();

        let thread = thread::spawn(move || handle_thread(counter_copy, prime_vectory_copy));
        thread_vector.push(thread);
    }

    for thread in thread_vector {
        thread.join().unwrap();
    }

    let duration = start.elapsed();

    let mut lock = prime_vector.lock().unwrap();

    let mut sum: u64 = 0;
    for num in lock.iter() {
        sum += *num;
    }

    lock.sort();

    let lower = lock.len() - 10;
    let upper = lock.len();

    let mut max_primes: Vec<u64> = Vec::new();

    for i in lower..upper {
        max_primes.push(*lock.get(i).unwrap());
    }

    fs::File::create("primes.txt").unwrap();

    let mut fi = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("primes.txt")
        .unwrap();
        
    fi.write_all(format!("{:?} {} {} {:?}", duration, lock.len(), sum, max_primes).as_bytes()).unwrap();
}

fn handle_thread(counter: Arc<Mutex<u64>>, prime_vector: Arc<Mutex<Vec<u64>>>) {
    let mut i: u64 = 0;

    while i < 100_000_000 {
        if is_prime(i) {
            prime_vector.lock().unwrap().push(i);
        }

        {
            let mut lock = counter.lock().unwrap();
            i = *lock;
            *lock += 1;
        }
    }
}

fn is_prime(num: u64) -> bool {
    if num == 2 || num == 3 {
        return true;
    }

    if num <= 1 || num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    return true;
}
