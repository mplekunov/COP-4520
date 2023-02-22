use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};

fn visit_room(
    guest: usize,
    max_guests: usize,
    visited: Arc<Mutex<HashSet<usize>>>,
    is_room_opened: Arc<Mutex<bool>>,
) {
    while visited.lock().unwrap().to_owned().len() < max_guests {
        if is_room_opened.lock().unwrap().to_owned() && !visited.lock().unwrap().contains(&guest) {
            *is_room_opened.lock().unwrap() = false;
            
            visited.lock().unwrap().insert(guest);

            *is_room_opened.lock().unwrap() = true;
        }
    }
}

fn main() {
    let start = Instant::now();

    let max_guests: usize = 100;

    let visited = Arc::new(Mutex::new(HashSet::<usize>::new()));
    let is_room_opened = Arc::new(Mutex::new(true));

    let mut threads = Vec::with_capacity(max_guests as usize);

    for guest in 0..max_guests as usize {
        let is_room_opened_clone = is_room_opened.clone();
        let visited_clone = visited.clone();
        threads.push(thread::spawn(move || visit_room(guest, max_guests, visited_clone, is_room_opened_clone)));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let duration = start.elapsed();

    println!("Time passed: {:?}", duration);

    let all_guests_visited_room = (0..max_guests).all(|guest| visited.lock().unwrap().contains(&guest));

    println!("Everyone saw the vase: {}", all_guests_visited_room);
}
