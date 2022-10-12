fn play_goat_bleet() {
    use soloud::*;
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../goat.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
s
fn main() {
    println!("Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats.");
    println!("You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat.");
    println!("He then says to you, \"Do you want to pick door No. 2?\" Is it to your advantage to switch your choice?");
    play_goat_bleet();

}
