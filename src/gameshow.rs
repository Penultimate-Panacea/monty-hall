use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

/// Runs a single game of the game show, the participant chooses a door within this function and the function returns if the participant wins or not.
/// While https://xkcd.com/1282 makes excellent points, here we take the original conceptualization of the problem where winning a goat is considered negative
///  # Arguments
/// 
///  * door_vec:vec = List of Doors
///  * 



/// Runs a single game of the game show, the participant chooses a door within this function and the function returns if the participant wins or not.
/// While https://xkcd.com/1282 makes excellent points, here we take the original conceptualization of the problem where winning a goat is considered negative
///  # Arguments
/// 
///  * door_vec:Vec<bool> = List of doors, where true represents the prize door
/// 
/// # Return
///  
/// * winner:bool = Whether or not the participant won the prize in this game.
///
fn run_game_no_change (door_vec:Vec<bool>) -> bool {
	let mut rng:ThreadRng = rand::thread_rng();
	let chosen_door = Uniform::from(0..door_vec.len()).sample(&mut rng);
	return door_vec[chosen_door]
}


/// Runs a single game of the game show, the participant chooses a door within this function and then discards that door, the participant then chooses a new door.
/// While https://xkcd.com/1282 makes excellent points, here we take the original conceptualization of the problem where winning a goat is considered negative
///  # Arguments
/// 
///  * door_vec:Vec<bool> = List of doors, where true represents the prize door
/// 
/// # Return
///  
/// * winner:bool = Whether or not the participant won the prize in this game.
///
fn run_game_change (door_vec:Vec<bool>) -> bool {
	let mut rng:ThreadRng = rand::thread_rng();
	let first_chosen_door = Uniform::from(0..=door_vec.len()).sample(&mut rng);
	let mut new_door_vec = door_vec.clone();
	new_door_vec.remove(first_chosen_door);
	let second_chosen_door = Uniform::from(0..=door_vec.len()).sample(&mut rng);
	return door_vec[second_chosen_door]
}
///
/// Sets up the doors with the goats and prizes
/// 
/// # Arguments
/// 
/// * input_doors = The number of doors to set up, if 0 doors are suggested, correct number to usize::MAX
/// 
/// # Return
/// 
/// * door_vec: Vec<bool> = Vector of doors set up where true represents the prize door.
/// 
fn stagehand (input_doors:usize) -> Vec<bool> {
	let mut num_doors = input_doors;
	if num_doors == 0 {num_doors = usize::MAX}
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
	let test_len = 0;
	let result_len = stagehand(test_len).len();
	assert_eq!(usize::MAX, result_len);
	let test_len = 0xfffe;
	let result_len = stagehand(test_len).len();
	assert_eq!(test_len, result_len);
}


pub fn gameshow(num_doors: usize, num_runs: usize, change: bool) -> usize {
	let mut won_games:usize = 0;
	let runs: Vec<Vec<bool>> = vec![stagehand(num_doors); num_runs];
	if change {
		ParallelIterator::for_each(IntoParallelIterator::into_par_iter(runs), |game| {
			let winner: bool = run_game_no_change(game);
			if winner { won_games += 1;}
		});
	}
	else {
		ParallelIterator::for_each(IntoParallelIterator::into_par_iter(runs), |game| {
			let winner: bool = run_game_no_change(game);
			if winner { won_games += 1;}
		});		
	}
	won_games
}
