pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    // play rounds where it invoke guesser each time
    let mut history = Vec::new();
    for i in 1..=32 {
        let guess = guesser.guess(&history); // why the [..]?
        if guess == answer {return Some(i)}
        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
                word: guess,
                mask: correctness
            });
    }
    None
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Correctness {
    ///Green
    Correct,
    ///Yellow
    Misplaced,
    ///Gray
    Wrong,
}
impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong;5];
        let mut used = [false; 5];
        // Mark things Green
        for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a ==g {
                c[i] = Correctness::Correct;
                used[i] = true;
            } 
        }
        //Mark things Yellow
        for (i,g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
            // already marked Green
                continue;
            } 
            if answer.chars().enumerate().any(|(i,a)| {
               if a==g && !used[i] {
                 used[i] = true;
                 return true
               } 
               false
            }) {
                 c[i] = Correctness::Misplaced;
            } 
        }
        c
    }    
    
}


pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
