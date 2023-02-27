use std::{collections::HashMap, borrow::Cow};

use crate::{Guesser, Guess, DICT, Word, Correctness};

pub struct Naive{
    remaining: HashMap<&'static Word, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(
                DICT.lines().map(
                    |line| {
                    let (word, count) = line.split_once(" ").expect("every line is word + space + occurance");
                    let count:usize = count.parse().expect("every count is a number");
                    (word.as_bytes().try_into().expect("5 letter words!"), count)
                    }
            ))       
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static Word,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> Word{
        if history.is_empty(){
            return *b"tares";
        }
        if let Some(last) = history.last(){
            // update self.remaining based on history
            self.remaining.retain(|word, _| last.matches(word));
        }
        let remaining_count: usize = self.remaining.iter().map(|(_, &count)|{count}).sum();
        let mut best:Option<Candidate>= None;

        for (&word, _) in &self.remaining {
            // - SUM_i p_i * log(p_i)
            let mut goodness = 0.0;
            for pattern in Correctness::patterns(){
                // considering a world where we did guess word and got pattern as the correctness.
                // Now compute what then is left
                let mut in_pattern_total = 0;
                for (candidate, count) in &self.remaining {
                    let g = Guess {
                        word:Cow::Owned(*word),
                        mask:pattern
                    };
                    if g.matches(candidate) {
                        in_pattern_total += count;
                    } 
                }
                if in_pattern_total == 0 {continue;}
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                goodness += -(p_of_this_pattern * p_of_this_pattern.log2());
            }
            if let Some(c) = best {
                // is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness });
                }
            } else {
                    best = Some(Candidate{ word, goodness});
                }
            }
        *best.unwrap().word
    }
}

