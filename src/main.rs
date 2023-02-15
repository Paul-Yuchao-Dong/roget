use roget::Wordle;
const GAMES: &str = include_str!("../answers.txt");
fn main() {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let mut guesser = roget::algorithms::Naive::new();
        w.play(answer, guesser);
    }
}