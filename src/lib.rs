use std::{collections::HashSet, borrow::Cow};

pub mod algorithms;
const DICT:&str = include_str!("../dictionary.txt");
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
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
        for i in 1..=7 {
            let guess = guesser.guess(&history); // why the [..]?
            if guess == answer {return Some(i)}
            assert!(self.dictionary.contains(&*guess), "guess '{}' is not in dictionary", guess);
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                    word: Cow::Owned(guess),
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
        for (i,(a,g)) in answer.bytes().zip(guess.bytes()).enumerate() {
            if a ==g {
                c[i] = Correctness::Correct;
                used[i] = true;
            } 
        }
        //Mark things Yellow
        for (i,g) in guess.bytes().enumerate() {
            if c[i] == Correctness::Correct {
            // already marked Green
                continue;
            } 
            if answer.bytes().enumerate().any(|(i,a)| {
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
    
    pub fn patterns()-> impl Iterator<Item = [Self; 5]>{
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        ).map(|(a,b,c,d,e)|[a,b,c,d,e])
    }
}


pub struct Guess<'a> {
    pub word: Cow<'a, str>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &str) -> bool {
        // if guess G gives mask C against answer A, then
        // guess A should also give mask C against answer G
        return Correctness::compute(word, &self.word) == self.mask;
    }
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
        use std::borrow::Cow;
        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {word: Cow::Borrowed($prev), mask: mask!($($mask )+) }.matches($next));
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {word: Cow::Borrowed($prev), mask: mask!($($mask )+) }.matches($next));
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