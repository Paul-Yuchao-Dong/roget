use std::collections::HashMap;

use crate::{Guesser, Guess, DICT, Correctness};

pub struct Naive{
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(
                DICT.lines().map(
                    |line| {
                    let (word, count) = line.split_once(" ").expect("every line is word + space + occurance");
                    let count:usize = count.parse().expect("every count is a number");
                    (word, count)
                    }
            ))       
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String{
        if let Some(last) = history.last(){
            // update self.remaining based on history
            self.remaining.retain(|word, _| last.matches(word));
        }
        let total_count: usize = self.remaining.iter().map(|(word, &count)|{count}).sum();
        let mut best:Option<Candidate>= None;

        for (&word, &count) in &self.remaining {
            // - SUM_i p_i * log(p_i)
            let p_word = count as f64 / total_count as f64;
            let goodness = -(p_word * p_word.log2());
            if let Some(c) = best {
                // is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate { word, count, goodness })
                }
            } else {
                best = Some(Candidate{ word, count, goodness})
            }
        }
        best.unwrap().word.to_string()
    }
}

