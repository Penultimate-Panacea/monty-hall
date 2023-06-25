#![warn(missing_debug_implementations)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

use std::{io::Read, thread, sync::{Arc, Mutex, Condvar}};


mod sound;
mod gameshow;

fn simulation(num_doors: usize, num_simulations: &usize, change_choice: bool) -> fraction::Fraction{
    let local_sims: usize = *num_simulations;
    let won_games = gameshow::gameshow(num_doors,local_sims,change_choice);
    let win_percent:fraction::Fraction = fraction::Fraction::new(won_games as u64, local_sims as u64);
    win_percent
}

    fn make_goat(){
        println!("(_(");
        println!("/_/'_____/)");
        println!("  |      |");
        println!("  |\"\"\"\"\"\"|");
    }

fn main() {
    let input_int = 50_000_000;
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("She then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    println!("Running {input_int} simulations");
    sound::play_goat_bleet();
    make_goat();
    let done = Arc::new((Mutex::new(false), Condvar::new()));
    // Spawn threads
    for i in 0..6 {
        let done_clone = Arc::clone(&done);
        thread::spawn(move || {
            // Perform simulation
            let result = match i {
                0 => simulation(3, &input_int, false),
                1 => simulation(3, &input_int, true),
                2 => simulation(5, &input_int, false),
                3 => simulation(5, &input_int, true),
                4 => simulation(100, &input_int, false),
                5 => simulation(100, &input_int, true),
                _ => unreachable!(),
            };
            
            // Print the result
            println!("{:#.5}", result);

            // Set the flag to true for the current thread
            let &(ref lock, ref cvar) = &*done_clone;
            let mut done = lock.lock().unwrap();
            *done = true;

            // Notify the main thread that this thread has completed
            cvar.notify_one();
        });
    }

    // Wait for all threads to complete
    let &(ref lock, ref cvar) = &*done;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }

    // All threads have completed, execute remaining code
    println!("Press ENTER to quit...");
    let mut buffer = [0u8];
    std::io::stdin().read_exact(&mut buffer).unwrap();
}

