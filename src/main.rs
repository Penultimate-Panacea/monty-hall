#![warn(missing_debug_implementations)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

use std::io::Read;


mod sound;
mod gameshow;

fn simulation(num_doors: usize, num_simulations: usize, change_choice: bool) -> fraction::Fraction{
    let won_games = gameshow::gameshow(num_doors,num_simulations,change_choice);
    let win_percent:fraction::Fraction = fraction::Fraction::new(won_games as u64, num_simulations as u64);
    win_percent
}


use simple_user_input::get_input;

// SOURCE: https://users.rust-lang.org/t/how-to-get-user-input/5176/7
mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}


    fn make_goat(){
        println!("(_(");
        println!("/_/'_____/)");
        println!("  |      |");
        println!("  |\"\"\"\"\"\"|");
    }

fn main() {
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("She then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    sound::play_goat_bleet();
    make_goat();
    let input_string:String = get_input("Enter number of simulations: MAX VALUE {2,147,483,647}");
    let input_int: usize = input_string.parse::<u32>().unwrap() as usize;
    println!("3 Doors, No Change Win rate: {:#.5}", simulation(3, input_int  , false)); 
    println!("3 Doors,  Change Win rate: {:#.5}", simulation(3, input_int , true)); 
    println!("5 Doors, No Change Win rate: {:#.5}", simulation(5, input_int , false)); 
    println!("5 Doors,  Change Win rate: {:#.5}", simulation(5, input_int , true)); 
    println!("100 Doors, No Change Win rate: {:#.5}", simulation(100, input_int, false)); 
    println!("100 Doors,  Change Win rate: {:#.5}", simulation(100, input_int, true));
    println!("Press ENTER to continue...");
    let buffer: &mut [u8; 1] = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}

