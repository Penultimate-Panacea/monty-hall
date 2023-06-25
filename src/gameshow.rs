use kdam::BarExt;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use kdam::tqdm;


/// Runs a single game of the game show, the participant chooses a door within this function and the function returns if the participant wins or not.
/// While <https://xkcd.com/1282> makes excellent points, here we take the original conceptualization of the problem where winning a goat is considered negative
///  # Arguments
/// 
///  * `door_vec:Vec<bool>` = List of doors, where true represents the prize door
/// 
/// # Return
///  
/// * `winner:bool` = Whether or not the participant won the prize in this game.
///
fn run_game_no_change (door_vec:&[bool]) -> bool {
	let mut rng:ThreadRng = rand::thread_rng();
	let chosen_door = Uniform::from(0..door_vec.len()).sample(&mut rng);
	door_vec[chosen_door]
}


/// Runs a single game of the game show, the participant chooses a door within this function and then discards that door, the participant then chooses a new door.
/// While <https://xkcd.com/1282> makes excellent points, here we take the original conceptualization of the problem where winning a goat is considered negative
///  # Arguments
/// 
///  * `door_vec:Vec<bool>` = List of doors, where true represents the prize door
/// 
/// # Return
///  
/// * `winner:bool` = Whether or not the participant won the prize in this game.
///
fn run_game_change (input_door_vec:Vec<bool>) -> bool {
	let mut door_vec = input_door_vec.clone();
	let mut rng:ThreadRng = rand::thread_rng();
	let first_chosen_door = Uniform::from(0..door_vec.len()).sample(&mut rng); // The Game Contestant Chooses a Door
	let mut revealed_door = first_chosen_door; // The Game Show Host reveals a Door
	while revealed_door == first_chosen_door && !door_vec[revealed_door] {
		revealed_door = Uniform::from(0..door_vec.len()).sample(&mut rng);
	} // Verify that the Chosen Door is not the prize door or the first chosen door.
	
	// The following if statements ensure the elements get removed in the correct order
	
	if first_chosen_door > revealed_door{door_vec.remove(first_chosen_door); door_vec.remove(revealed_door);}
	if first_chosen_door < revealed_door{door_vec.remove(revealed_door); door_vec.remove(first_chosen_door);}
	
	let second_chosen_door = Uniform::from(0..door_vec.len()).sample(&mut rng);
	door_vec[second_chosen_door]
}
///
/// Sets up the doors with the goats and prizes
/// 
/// # Arguments
/// 
/// * `input_doors: usize` = The number of doors to set up, if 0 doors are suggested, correct number to `usize::MAX`
/// 
/// # Return
/// 
/// * `door_vec: Vec<bool>` = Vector of doors set up where true represents the prize door.
/// 
fn stagehand (input_doors:usize) -> Vec<bool> {
	let mut num_doors = input_doors;
	if num_doors == 0 {num_doors = 0xffff}
	let mut doors: Vec<bool> = vec![false; num_doors];
	let mut rng:ThreadRng = rand::thread_rng();
	let prize_door = Uniform::from(0..num_doors).sample(&mut rng);
	doors[prize_door] = true;
	doors
}

#[test]
fn test_stagehand_lengths() {
	let test_len = 3;
	let result_len = stagehand(test_len).len();
	assert_eq!(test_len, result_len);
	let test_len = 0xffff;
	let result_len = stagehand(test_len).len();
	assert_eq!(test_len, result_len);
}


#[test]
fn test_stagehand_one_prize_door(){
	let door_vec: Vec<bool> = stagehand(100_000);
	let mut prize_doors: Vec<bool> = vec![];
	for door in door_vec {
		if door == true{
			prize_doors.push(true);
		}
	}
	assert_eq!(prize_doors.len(), 1);

}
pub fn gameshow(num_doors: usize, num_runs: usize, change: bool) -> usize {
	let mut won_games: usize = 0;
	let runs: Vec<Vec<bool>> = vec![stagehand(num_doors); num_runs]; // Creates common solution 
	let mut progressbar = tqdm!(total = runs.len());
	if change {
		for game in runs{
			let winner: bool = run_game_change(game); // Each door is chosen randomly when tested
			if winner { won_games += 1;}
			progressbar.update(1);
		};
	}
	else {
		for game in runs {
			let winner: bool = run_game_no_change(&game); // Each door is chosen randomly when tested
			if winner {won_games += 1;}
			progressbar.update(1);
		};		
	}
	eprint!("\n");
	won_games
}
