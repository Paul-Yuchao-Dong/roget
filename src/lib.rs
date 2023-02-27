use std::{collections::HashSet, borrow::Cow};

pub mod algorithms;
const DICT:&str = include_str!("../dictionary.txt");
pub type Word = [u8; 5];
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> Word;
}
pub struct Wordle{
    dictionary: HashSet<&'static Word>
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
                    .as_bytes()
                    .try_into()
                    .expect("Every word should be 5 characters")
                ) )       
            }
    }
    pub fn play<G: Guesser>(&self, answer: Word, mut guesser: G) -> Option<usize> {
        // play rounds where it invoke guesser each time
        let mut history = Vec::new();
        for i in 1..=7 {
            let guess = guesser.guess(&history); // why the [..]?
            if guess == answer {return Some(i)}
            assert!(self.dictionary.contains(&guess), 
                    "guess '{}' is not in dictionary", 
                    std::str::from_utf8(&guess).unwrap());
            let correctness = Correctness::compute(&answer, &guess);
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
    fn compute(answer: &Word, guess: &Word) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong;5];
        let mut used = [false; 5];
        // Mark things Green
        for (i,(a,g)) in answer.iter().zip(guess.iter()).enumerate() {
            if a ==g {
                c[i] = Correctness::Correct;
                used[i] = true;
            } 
        }
        //Mark things Yellow
        for (i,g) in guess.iter().enumerate() {
            if c[i] == Correctness::Correct {
            // already marked Green
                continue;
            } 
            if answer.iter().enumerate().any(|(i,a)| {
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
    pub word: Cow<'a, Word>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &Word) -> bool {
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
            fn guess(&mut self, $history: &[Guess]) -> $crate::Word {
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
            check!(b"abcde" + [C C C C C] allows b"abcde");
            check!(b"abcdf" + [C C C C C] disallows b"abcde");
            check!(b"abcde" + [W W W W W] allows b"fghij");
            check!(b"abcde" + [M M M M M] allows b"bcdea");

        }
        #[test]
        fn chat(){
            check!(b"aaabb" + [C M W W W] disallows b"accaa");
            check!(b"baaaa" + [W C M W W] allows b"aaccc");
            check!(b"baaaa" + [W C M W W] disallows b"caacc");
        }
        #[test]
        fn debug(){
            check!(b"baaaa" + [W C M W W] allows b"aaccc");
        }
    }
    mod game {
        use crate::{ Wordle, Guess};

        #[test]
        fn genius() {
            let w= Wordle::new();
            let guesser = guesser!(|_history| {*b"moved"} );
            let tmp = w.play(*b"moved", guesser);
            assert_eq!(tmp, Some(1));
        }
        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return b"right".to_owned();
                }
                return b"wrong".to_owned();
            } );
            let tmp = w.play(*b"right", guesser);
            assert_eq!(tmp, Some(2));
        }
        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {return *b"wrong";} );
            let tmp = w.play(*b"right", guesser);
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
                Correctness::compute(b"abcde", b"abcde"),
                mask![C C C C C]
            )
        }
        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::compute(b"abcde", b"abcde"),
                [Correctness::Correct;5]
            )
        }
        #[test]
        fn all_gray() {
            assert_eq!(
                Correctness::compute(b"abcde", b"fghij"),
                [Correctness::Wrong;5]
            )
        }
        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::compute(b"abcde", b"bcdea"),
                mask!(M M M M M)
            )
        }
        #[test]
        fn repeat_green() {
            assert_eq!(
                Correctness::compute(b"aabbb", b"aaccc"),
                mask!(C C W W W)
            )
        }
        #[test]
        fn repeat_yellow() {
            assert_eq!(
                Correctness::compute(b"aabbb", b"ccaac"),
                mask!(W W M M W)
            )
        }
        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute(b"aabbb", b"caacc"),mask!(W C M W W))
        }
        #[test]
        fn chat1() {
            assert_eq!(Correctness::compute(b"azzaz", b"aaabb"),mask!(C M W W W))
        }
        #[test]
        fn chat2() {
            assert_eq!(Correctness::compute(b"baccc", b"aaddd"),mask!(W C W W W))
        }
        #[test]
        fn chat3() {
            assert_eq!(Correctness::compute(b"abcde", b"aacde"),mask!(C W C C C))
        }

    }
}