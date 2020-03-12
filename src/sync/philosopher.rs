use std::sync::{Arc, Mutex};

use super::semaphore::Semaphore;

pub fn solution() {
    let mut chopsticks = vec![];
    for _ in 0..5 {
        chopsticks.push(Semaphore::new(1));
    }
    let chopsticks = Arc::new(Mutex::new(chopsticks));
    let philosophers = (0..5)
        .map(|i| {
            let chopsticks = chopsticks.clone();
            std::thread::spawn(move || {
                for j in 1..=3 {
                    println!("Philosopher[{}] is Thinking {}th time", i, j);
                    let chopsticks = chopsticks.lock().expect("failed to lock");
                    chopsticks[i].acquire();
                    chopsticks[(i + 1) % 5].acquire();
                    println!("Philosopher[{}] is Eating {}th time", i, j);
                    chopsticks[i].release();
                    chopsticks[(i + 1) % 5].release();
                }
            })
        })
        .collect::<Vec<_>>();
    for p in philosophers {
        p.join().expect("philosopher failed");
    }
}
