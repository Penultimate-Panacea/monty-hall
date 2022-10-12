// use percentage::Percentage;
// use percentage::PercentageDecimal;
// use rand::Rng;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use soloud::*;

#[derive(Debug, Clone)]
struct Door {
    prize: bool,
    chosen: bool,
    opened: bool
}

struct GameshowResult{
    winner: bool
}

fn play_goat_bleet() {
    
    let sl:Soloud = Soloud::default().unwrap();
    let mut wav:Wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../goat.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}

fn game_show(num: i64, change_choice: bool) -> GameshowResult {
    let mut rng:ThreadRng = rand::thread_rng();
    let mut doors:Vec<Door> = Vec::new();  
    log::debug!("Populating Door Vec");
    for _a in 0..num{
        doors.push(Door{prize: false, chosen: false, opened: false});
    }
    let random_door = Uniform::from(0..num);
    log::debug!("Choosing Prize Door");
    
    let prize_door = random_door.sample(&mut rng);
    doors[prize_door as usize].prize = true;
    log::debug!("Prize door is: {}", prize_door);

    log::debug!("Choosing Initial Door"); 
    let chosen_door = random_door.sample(&mut rng);
    doors[chosen_door as usize].chosen = true;
    log::debug!("Chosen door is: {}", chosen_door);
    
    if change_choice {
        log::debug!("Opening Other Doors");
        for mut i in &mut doors {
            if i.chosen == false && i.prize == false  {i.opened = true;}
        }
        // let mut new_door_list:Vec<Door> = Vec::new(); 
        // for j in doors{if j.opened == false && j.chosen == false {new_door_list.push(j.clone());}}
        return new_choice(doors, prize_door)
    }

    if chosen_door == prize_door {
        return GameshowResult{winner: true}
    }
    else {
        return GameshowResult{winner: false}
    }

}

fn new_choice(doors: Vec<Door>, prize_door: i64) -> GameshowResult{
    let mut new_door_list:Vec<Door> = Vec::new(); 
    let mut rng:ThreadRng = rand::thread_rng();
    for door in doors {if door.opened == false && door.chosen == false {new_door_list.push(door);}} // Remove opened doors from options and change from the previously chosen door
    if new_door_list.len() == 0 {return  GameshowResult{winner:true}}//If originally chosen door was correct, return false as the door would be changed
    let random_door = Uniform::from(0..new_door_list.len());
    let newly_chosen_door: i64 = random_door.sample(&mut rng) as i64;
    if newly_chosen_door == prize_door {
        return GameshowResult{winner: true}
    }
    else {
        return GameshowResult{winner: false}
    }
}

fn simulation(num_doors: i64, num_simulations: u32, change_choice: bool) -> fraction::Fraction{
    let mut simulations_remaining = num_simulations;
    let mut won_games = 0;
    let mut simulations_completed = 0;
    // let mut rng:ThreadRng = rand::thread_rng();
    while simulations_remaining != 0{
        let game = game_show(num_doors, change_choice);
        simulations_completed = simulations_completed + 1;
        if game.winner {won_games = won_games + 1};
        simulations_remaining = simulations_remaining - 1;
    }
    // let win_percent = Percentage::from_decimal(won_games as f64 / num_simulations as f64);
    let win_percent:fraction::Fraction = fraction::Fraction::new(won_games as u32, num_simulations);
    println!("Sims Complete: {}", simulations_completed);
    return win_percent;
}


fn main() {
    env_logger::init();
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("She then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    play_goat_bleet();
    println!("Win rate: {}", simulation(3, i16::MAX as u32 , false)); 
    println!("Win rate: {}", simulation(3, i16::MAX as u32, true)); 
    println!("Win rate: {}", simulation(5, i16::MAX as u32 , false)); 
    println!("Win rate: {}", simulation(5, i16::MAX as u32, true)); 
    println!("Win rate: {}", simulation(100, i16::MAX as u32, false)); 
    println!("Win rate: {}", simulation(100, i16::MAX as u32, true)); 
    
}