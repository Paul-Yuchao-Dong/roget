use std::collections::HashSet;

pub mod algorithms;
const DICT:&str = include_str!("../dictionary.txt");
pub struct Wordle{
    dictionary: HashSet<&'static str>
}

impl Wordle {
    pub fn new()-> Self{
        Self {
            dictionary:HashSet::from_iter(
                DICT.lines().map(
                    |line| 
                    line
                    .split_once(" ")
                    .expect("every line is word + space + occurance")
                    .0
                ) )       
            }
    }
    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        // play rounds where it invoke guesser each time
        let mut history = Vec::new();
        for i in 1..=32 {
            let guess = guesser.guess(&history); // why the [..]?
            if guess == answer {return Some(i)}
            assert!(self.dictionary.contains(&*guess), "guess '{}' is not in dictionary", guess);
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                    word: guess,
                    mask: correctness
                });
        }
        None
    }
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
    
    pub fn permutations()-> impl Iterator<Item = [Self; 5]>{
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        ).map(|(a,b,c,d,e)|[a,b,c,d,e])
    }
}


pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        
        // First check Greens
        let mut used = [false; 5];
        for (i,((g,&m),w)) in self.word.chars().zip(&self.mask).zip(word.chars()).enumerate(){
            if m==Correctness::Correct{
                if g!=w {return false;} else {
                    used[i] =true;
                    continue;
                }
            }
        }

        for (i,(w,m)) in word
                                    .chars()
                                    .zip(&self.mask)
                                    .enumerate()
        {    
            if *m == Correctness::Correct {continue;} // must have been correct, or we'd returned
            let mut plausible = true; 
            if self.word.chars().zip(&self.mask).enumerate().any(|(j,(g,m))| {
                if g!=w {return false;}
                if used[j] {
                    // Can't use this to support the character
                    return false;
                } 
                // we are looking at an w in word, and have found an w in previous guess.
                // the color of that previous w will tell us whether this w _might_ be okay.
                match m {
                    Correctness::Correct => unreachable!("all correct guesses should have resulted in return or be used"),
                    Correctness::Misplaced if j==i => {
                        // w was in the same position last time around, which means word cannot be the answer
                        plausible = false;
                        return false;
                    },
                    Correctness::Misplaced => {
                        used[j] = true;
                        return true
                    },
                    Correctness::Wrong => {
                        // TODO early return
                        // dbg!(i, j,g,m,w);
                        plausible = false;
                        return false;
                    },
                }
            } ) && plausible {
                // the charactor w was yellow in the previous match
            } else if !plausible {
                return false;
            } else {
            }
        }
        true
    }
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
#[cfg(test)]
macro_rules! guesser {
    (|$history: ident| $impl: block) => {{
        struct G;
        impl crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G        
    }};
}

#[cfg(test)]
macro_rules! mask {
    (M) => {crate::Correctness::Misplaced};
    (C) => {crate::Correctness::Correct};
    (W) => {crate::Correctness::Wrong};
    ($($c:tt)+) => {[$(mask!($c)),+]};
}
#[cfg(test)]
mod tests {
    mod guess_matcher{
        use crate::Guess;
        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {word: $prev.to_string(), mask: mask!($($mask )+) }.matches($next));
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {word: $prev.to_string(), mask: mask!($($mask )+) }.matches($next));
            };
        }
        #[test]
        fn matches() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("abcde" + [W W W W W] allows "fghij");
            check!("abcde" + [M M M M M] allows "bcdea");

        }
        #[test]
        fn chat(){
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("baaaa" + [W C M W W] allows "aaccc");
            check!("baaaa" + [W C M W W] disallows "caacc");
        }
        #[test]
        fn debug(){
            check!("baaaa" + [W C M W W] allows "aaccc");
        }
    }
    mod game {
        use crate::{ Wordle, Guess};

        #[test]
        fn genius() {
            let w= Wordle::new();
            let guesser = guesser!(|_history| {"moved".to_string()} );
            let tmp = w.play("moved", guesser);
            assert_eq!(tmp, Some(1));
        }
        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            } );
            let tmp = w.play("right", guesser);
            assert_eq!(tmp, Some(2));
        }
        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {return "wrong".to_string();} );
            let tmp = w.play("right", guesser);
            assert_eq!(tmp, None);
        }
    }
    mod compute {
        use crate::Correctness;
        macro_rules! mask {
            (M) => {Correctness::Misplaced};
            (C) => {Correctness::Correct};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[$(mask!($c)),+]};
        }
        #[test]
        fn basic() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                mask![C C C C C]
            )
        }
        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                [Correctness::Correct;5]
            )
        }
        #[test]
        fn all_gray() {
            assert_eq!(
                Correctness::compute("abcde", "fghij"),
                [Correctness::Wrong;5]
            )
        }
        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::compute("abcde", "bcdea"),
                mask!(M M M M M)
            )
        }
        #[test]
        fn repeat_green() {
            assert_eq!(
                Correctness::compute("aabbb", "aaccc"),
                mask!(C C W W W)
            )
        }
        #[test]
        fn repeat_yellow() {
            assert_eq!(
                Correctness::compute("aabbb", "ccaac"),
                mask!(W W M M W)
            )
        }
        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"),mask!(W C M W W))
        }
        #[test]
        fn chat1() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"),mask!(C M W W W))
        }
        #[test]
        fn chat2() {
            assert_eq!(Correctness::compute("baccc", "aaddd"),mask!(W C W W W))
        }
        #[test]
        fn chat3() {
            assert_eq!(Correctness::compute("abcde", "aacde"),mask!(C W C C C))
        }

    }
}