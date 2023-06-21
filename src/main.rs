#![warn(missing_debug_implementations)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

use std::io::Read;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use tqdm::tqdm;

mod sound;



#[derive(Debug, Clone)]
struct Door {
    prize: bool,
    chosen: bool,
    opened: bool
}

struct GameshowResult{
    winner: bool
}

fn game_show(num: usize, change_choice: bool) -> GameshowResult {
    let mut rng:ThreadRng = rand::thread_rng();
    let mut doors:Vec<Door> = Vec::new();  
    log::debug!("Populating Door Vec");
    for _a in 0..num{
        doors.push(Door{prize: false, chosen: false, opened: false});
    }
    let random_door = Uniform::from(0..num);
    log::debug!("Choosing Prize Door");
    
    let prize_door = random_door.sample(&mut rng);
    doors[prize_door].prize = true;
    log::debug!("Prize door is: {}", prize_door);

    log::debug!("Choosing Initial Door"); 
    let chosen_door = random_door.sample(&mut rng);
    doors[chosen_door].chosen = true;
    log::debug!("Chosen door is: {}", chosen_door);
    
    if change_choice {
        log::debug!("Opening Other Doors");
        for mut i in &mut doors {
            if !i.chosen && !i.prize   {i.opened = true;}
        }
        return new_choice(doors, prize_door)
    }

    if chosen_door == prize_door {
        return GameshowResult{winner: true}
    }
    GameshowResult{winner: false}
}

fn new_choice(doors: Vec<Door>, prize_door: usize) -> GameshowResult{
    let mut new_door_list:Vec<Door> = Vec::new(); 
    let mut rng:ThreadRng = rand::thread_rng();
    for door in doors {if !door.opened && !door.chosen{new_door_list.push(door);}} // Remove opened doors from options and change from the previously chosen door
    if new_door_list.is_empty() {return  GameshowResult{winner:true}}//If only option wins, then do not randomize, simply return win
    let random_door = Uniform::from(0..new_door_list.len());
    let newly_chosen_door: usize = random_door.sample(&mut rng);
    if newly_chosen_door == prize_door {
        return GameshowResult{winner: true}
    }
    GameshowResult{winner: false}
}

fn simulation(num_doors: usize, num_simulations: u32, change_choice: bool) -> fraction::Fraction{
    let mut won_games:u32 = 0;
    for _ in tqdm(0..num_simulations){
        let game = game_show(num_doors, change_choice);
        if game.winner {won_games += 1};
    }

    let win_percent:fraction::Fraction = fraction::Fraction::new(won_games, num_simulations);
    win_percent
}


use simple_user_input::get_input;

// SOURCE: https://users.rust-lang.org/t/how-to-get-user-input/5176/7
mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{prompt}");
        let mut input = String::new();
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
    env_logger::init();
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("She then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    sound::play_goat_bleet();
    make_goat();
    let input_string:String = get_input("Enter number of simulations: MAX VALUE {2,147,483,647}");
    let input_int: u32 = input_string.parse::<u32>().unwrap();
    println!("3 Doors, No Change Win rate: {:#.3}", simulation(3, input_int  , false)); 
    println!("3 Doors,  Change Win rate: {:#.3}", simulation(3, input_int , true)); 
    println!("5 Doors, No Change Win rate: {:#.3}", simulation(5, input_int , false)); 
    println!("5 Doors,  Change Win rate: {:#.3}", simulation(5, input_int , true)); 
    println!("100 Doors, No Change Win rate: {:#.3}", simulation(100, input_int, false)); 
    println!("100 Doors,  Change Win rate: {:#.3}", simulation(100, input_int, true)); 
    println!("Press ENTER to continue...");
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();

}