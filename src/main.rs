use std::{
    fs,
    io::Write,
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant
};

const MAX_NUM: u64 = 100_000_000;
const NUM_THREADS: u32 = 8;

fn main() {
    let start = Instant::now();

    let counter = Arc::new(Mutex::new(1));
    let prime_vector: Arc<Mutex<Vec<bool>>> =
        Arc::new(Mutex::new(vec![false; (MAX_NUM + 1) as usize]));
    let residual_primes_found = Arc::new(Mutex::new(false));

    let mut thread_vector = Vec::new();

    for _ in 0..NUM_THREADS {
        let counter_copy = counter.clone();
        let prime_vector_copy = prime_vector.clone();
        let residual_primes_found_copy = residual_primes_found.clone();

        let thread = thread::spawn(move || {
            sieve_of_atking(
                counter_copy,
                prime_vector_copy,
                residual_primes_found_copy,
                MAX_NUM,
            );
        });

        thread_vector.push(thread);
    }

    for thread in thread_vector {
        thread.join().unwrap();
    }

    let duration = start.elapsed();

    let prime_vector_copy = prime_vector.lock().unwrap().clone();

    let mut primes: Vec<u64> = Vec::new();
    for val in 0..prime_vector_copy.len() {
        if prime_vector_copy[val] {
            primes.push(val as u64);
        }
    }

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

fn sieve_of_eratosthenes(
    counter: Arc<Mutex<u64>>,
    prime_vector: Arc<Mutex<Vec<bool>>>,
    num: u64
) {
    {
        prime_vector.lock().unwrap()[0] = false;
        prime_vector.lock().unwrap()[1] = false;
    }

    let mut p: u64;

    {
        let mut counter_locked = counter.lock().unwrap();
        p = *counter_locked;
        *counter_locked += 1;
    }

    while p * p <= num  {
        if prime_vector.lock().unwrap()[p as usize] {
            let mut i = p * p;

            while i <= num {
                prime_vector.lock().unwrap()[i as usize] = false;
                i += p;
            }
        }
     
        {
            let mut counter_locked = counter.lock().unwrap();
            p = *counter_locked;
            *counter_locked += 1;
        }
    }
}

fn sieve_of_atking(
    counter: Arc<Mutex<u64>>,
    prime_vector: Arc<Mutex<Vec<bool>>>,
    residual_primes_found: Arc<Mutex<bool>>,
    num: u64,
) {
    if num > 2 {
        prime_vector.lock().unwrap()[2] = true;
    }

    if num > 3 {
        prime_vector.lock().unwrap()[3] = true;
    }

    let mut x;

    {
        let mut counter_locked = counter.lock().unwrap();
        x = *counter_locked;
        *counter_locked += 1;
    }

    let mut index_vector: Vec<usize> = Vec::new();

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
            *counter_locked += 1;
        }
    }

    for index in index_vector {
        prime_vector.lock().unwrap()[index] ^= true;
    }

    if !*residual_primes_found.lock().unwrap() {
        {
            *residual_primes_found.lock().unwrap() = true;
        }

        let mut r = 5;

        while r * r <= num {
            if prime_vector.lock().unwrap()[r as usize] {
                let mut i = r * r;

                while i <= num {
                    prime_vector.lock().unwrap()[i as usize] = false;

                    i += r * r;
                }
            }

            r += 1;
        }
    }
}
