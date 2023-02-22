use std::{
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};

use rand::Rng;

fn generate_number(from: i64, to: i64) -> i64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(from..to);
}

fn renew_cupcakes(
    guest_id: u32,
    max_guests: u32,
    cupcakes_distributed: Arc<Mutex<u32>>,
    is_cupcake_available: Arc<Mutex<bool>>,
    current_guest: Arc<Mutex<u32>>,
    guests_with_cupcakes: Arc<Mutex<Vec<bool>>>,
) {
    while cupcakes_distributed.lock().unwrap().lt(&max_guests) {
        let cupcakes_distributed_clone = cupcakes_distributed.clone();
        let is_cupcake_available_clone = is_cupcake_available.clone();
        let current_guest_clone = current_guest.clone();
        let guests_with_cupcakes_clone = guests_with_cupcakes.clone();

        if current_guest_clone.lock().unwrap().eq(&guest_id) {
            if !is_cupcake_available_clone.lock().unwrap().to_owned() {
                *cupcakes_distributed_clone.lock().unwrap() += 1;
                *is_cupcake_available_clone.lock().unwrap() = true;
            }

            if !guests_with_cupcakes_clone.lock().unwrap()[guest_id as usize] {
                try_eating_cupcake(
                    guest_id,
                    is_cupcake_available_clone,
                    guests_with_cupcakes_clone,
                );
            }
        }
    }
}

fn try_eating_cupcake(
    guest_id: u32,
    is_cupcake_available: Arc<Mutex<bool>>,
    guests_with_cupcakes: Arc<Mutex<Vec<bool>>>,
) {
    if is_cupcake_available.lock().unwrap().to_owned()
        && !guests_with_cupcakes.lock().unwrap()[guest_id as usize].to_owned()
    {
        guests_with_cupcakes.lock().unwrap()[guest_id as usize] = true;
        *is_cupcake_available.lock().unwrap() = false;
    }
}

fn explore_labyrinth(
    guest_id: u32,
    max_guests: u32,
    cupcakes_distributed: Arc<Mutex<u32>>,
    is_cupcake_available: Arc<Mutex<bool>>,
    current_guest: Arc<Mutex<u32>>,
    guests_with_cupcakes: Arc<Mutex<Vec<bool>>>,
) {
    while cupcakes_distributed.lock().unwrap().lt(&max_guests) {
        let is_cupcake_available_clone = is_cupcake_available.clone();
        let guests_with_cupcakes_clone = guests_with_cupcakes.clone();

        if current_guest.lock().unwrap().eq(&guest_id) {
            try_eating_cupcake(
                guest_id,
                is_cupcake_available_clone,
                guests_with_cupcakes_clone,
            );
        }
    }
}

fn main() {
    let start = Instant::now();

    let max_guests: u32 = 50;

    let cupcakes_distributed = Arc::new(Mutex::new(0));
    let is_cupcake_available = Arc::new(Mutex::new(true));

    let guests_with_cupcakes = Arc::new(Mutex::new(vec![false; max_guests as usize]));

    let current_guest = Arc::new(Mutex::new(0));

    let mut threads = Vec::with_capacity(max_guests as usize);

    for i in 0..max_guests {
        let cupcakes_distributed_clone = cupcakes_distributed.clone();
        let is_cupcake_available_clone = is_cupcake_available.clone();
        let current_guest_clone = current_guest.clone();
        let guests_with_cupcakes_clone = guests_with_cupcakes.clone();

        let join_handler = if i == 0 {
            thread::spawn(move || {
                renew_cupcakes(
                    0,
                    max_guests,
                    cupcakes_distributed_clone,
                    is_cupcake_available_clone,
                    current_guest_clone,
                    guests_with_cupcakes_clone,
                )
            })
        } else {
            thread::spawn(move || {
                explore_labyrinth(
                    i as u32,
                    max_guests as u32,
                    cupcakes_distributed_clone,
                    is_cupcake_available_clone,
                    current_guest_clone,
                    guests_with_cupcakes_clone,
                )
            })
        };

        threads.push(join_handler);
    }

    while cupcakes_distributed.lock().unwrap().lt(&max_guests) {
        *current_guest.lock().unwrap() = generate_number(0, max_guests as i64) as u32;
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let duration = start.elapsed();

    println!("Time passed: {:?}", duration);

    let all_cupcakes_distributed = guests_with_cupcakes
        .lock()
        .unwrap()
        .iter()
        .all(|&x| x);

    println!("Everyone got a cupcake {}", all_cupcakes_distributed);
}
