use std::{
    fs,
    io::Write,
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant, ops::AddAssign,
};

const MAX_NUM: u64 = 100_000_000;
const NUM_THREADS: u32 = 8;

fn main() {
    let start = Instant::now();

    let mut primes = sieve_of_atking(NUM_THREADS, MAX_NUM);

    let duration = start.elapsed();

    let mut sum: u64 = 0;

    for num in primes.iter() {
        sum += *num;
    }

    primes.sort();

    let lower = primes.len() - 10;
    let upper = primes.len();

    let mut max_primes: Vec<u64> = Vec::new();

    for i in lower..upper {
        max_primes.push(*primes.get(i).unwrap());
    }

    write_to_file(
        "primes.txt",
        format!("{:?} {} {} {:?}", duration, primes.len(), sum, max_primes).as_bytes(),
    );
}

fn write_to_file(file_name: &str, data: &[u8]) {
    fs::File::create(file_name).unwrap();

    let mut fi = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    fi.write_all(data).unwrap();
}

fn sieve_of_atking(num_threads: u32, num: u64) -> Vec<u64> {
    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));

    let mut sieve_primes: Vec<bool> = vec![false; (num + 1) as usize];

    if num >= 2 {
        sieve_primes[2] = true;
    }

    if num >= 3 {
        sieve_primes[3] = true;
    }

    let mut threads = Vec::with_capacity(num_threads as usize);

    for _ in 0..num_threads {
        let counter_copy = counter.clone();

        threads.push(thread::spawn(move || {
            return sieve_of_atking_thread(counter_copy, num);
        }));
    }

    for thread in threads {
        let index_vector = &thread.join().unwrap();

        index_vector.iter().for_each(|index| sieve_primes[*index] ^= true);
    }

    let mut r = 5;

    while r * r <= num {
        if sieve_primes[r as usize] {
            let mut i = r * r;

            while i <= num {
                sieve_primes[i as usize] = false;

                i += r * r;
            }
        }

        r += 1;
    }

    let mut primes: Vec<u64> = Vec::with_capacity((num / 2) as usize);

    sieve_primes.iter().enumerate().for_each(| (index, val) | if *val { primes.push(index as u64); });

    return primes;
}

fn sieve_of_atking_thread(counter: Arc<Mutex<u64>>, num: u64) -> Vec<usize> {
    let mut index_vector: Vec<usize> = Vec::with_capacity((num / 2) as usize);

    let mut x;

    {
        let mut counter_locked = counter.lock().unwrap();
        x = *counter_locked;
        (*counter_locked).add_assign(1);
    }

    while x * x <= num {
        let mut y = 1;

        while y * y <= num {
            let mut n = (4 * x * x) + (y * y);
            if n <= num && (n % 12 == 1 || n % 12 == 5) {
                index_vector.push(n as usize);
            }

            n = (3 * x * x) + (y * y);
            if n <= num && n % 12 == 7 {
                index_vector.push(n as usize);
            }

            n = (3 * x * x) - (y * y);
            if x > y && n <= num && n % 12 == 11 {
                index_vector.push(n as usize);
            }

            y += 1;
        }

        {
            let mut counter_locked = counter.lock().unwrap();
            x = *counter_locked;
            (*counter_locked).add_assign(1);
        }
    }

    return index_vector;
}
