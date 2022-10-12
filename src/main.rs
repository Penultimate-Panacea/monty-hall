use std::i128;

use percentage::Percentage;
use rand::Rng;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use soloud::*;

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

fn game_show(num: i128, change_choice: bool) -> GameshowResult {
    let mut rng:ThreadRng = rand::thread_rng();
    let mut doors:Vec<Door> = Vec::new();  
    println!("Populating Door Vec");
    for door in 0..num{
        doors.push(Door{prize: false, chosen: false, opened: false});
    }
    let random_door = Uniform::from(0..num);
    println!("Choosing Prize Door");
    
    let prize_door = random_door.sample(&mut rng);
    doors[prize_door].prize = true;
    println!("Prize door is: {}", prize_door);

    println!("Choosing Initial Door");
    let chosen_door = random_door.sample(&mut rng);
    doors[chosen_door].chosen = true;
    println!("Chosen door is: {}", chosen_door);
    
    // if change_choice {
    //     println!("Opening Other Doors");
    //     for mut door in doors{
    //     if (door.chosen == false && door.prize == false ) {
    //         door.opened = true; 
    //     }
    // }

    if (chosen_door == prize_door) {
        return GameshowResult{winner: true}
    }
    else {
        return GameshowResult{winner: false}
    }

}

fn simulation(num_doors: i128, num_simulations: i128, change_choice: bool) -> Percentage{
    
}


fn main() {
;
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("She then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    play_goat_bleet();
    3doors_nochange = game_show(3, false);
    3doors_change = game_show(3, true);

}


// let base_example_door_num:i8 = 3;
// let second_example_door_num:i8 = 4;
// let many_doors:i128 = i128::MAX;