use roget::Wordle;
const GAMES: &str = include_str!("../answers.txt");
fn main() {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let mut guesser = roget::algorithms::Naive::new();
        if let Some(score) = w.play(answer, guesser){
            println!("Guessed {} in {}", answer, score);
        }else{
            eprintln!("failed to guess ");
        };
    }
}