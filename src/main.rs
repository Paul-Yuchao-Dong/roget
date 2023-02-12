const GAMES: &str = include_str!("../answers.txt");
fn main() {
    let mut guesser = Naive::new();
    for answer in GAMES.split_whitespace() {
        play(answer, guesser);
    }
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {
    // play rounds where it invoke guesser each time


}

enum Correctness {
    ///Green
    Correct,
    ///Yellow
    Misplaced,
    ///Gray
    Wrong,
}

struct Guess {
    word: String,
    mask: [Correctness; 5],

}
trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl <T> Guesser for &mut T where T:Guesser{
    fn guess(&mut self, history: &[Guess]) -> String;
}