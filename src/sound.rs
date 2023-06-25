use soloud::{AudioExt, LoadExt, Soloud, Wav, audio};
///
/// Plays the sound of a goat bleeting for fun. 
///  # Arguments
/// 
///  * None
/// 
///  # Todo
///  Maybe take an argument for different goat bleets?
///     Replace soloud with more native rust lib
/// 
pub fn play_goat_bleet() {
    let sl:Soloud = Soloud::default().unwrap();
    let mut wav:Wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../goat.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
fn test_goat_bleet(){
	play_goat_bleet();
}