use crate::{Guesser, Guess, DICT, Correctness};
use std::{borrow::Cow};
use once_cell::sync::OnceCell;

static INITIAL: OnceCell<Vec<(&'static str, usize)>> = OnceCell::new();

pub struct Once{
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
}

impl Once {
    pub fn new() -> Self {
        Once {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(
                    DICT.lines().map(
                        |line| {
                        let (word, count) = line.split_once(" ").expect("every line is word + space + occurance");
                        let count:usize = count.parse().expect("every count is a number");
                        (word, count)
                        }
                )) 
            }))       
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Once {
    fn guess(&mut self, history: &[Guess]) -> String{
        if history.is_empty(){
            return "tares".to_owned();
        }
        if let Some(last) = history.last(){
            // update self.remaining based on history

            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining.to_mut().retain(|(word, _)| last.matches(word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                    .iter()
                    .filter(|(word, _)| last.matches(word))
                    .copied()
                    .collect()
                )
            }
        }
        let remaining_count: usize = self.remaining.iter().map(|(word, count)|{count}).sum();
        let mut best:Option<Candidate>= None;

        for &(word, _) in &*self.remaining {
            // - SUM_i p_i * log(p_i)
            let mut goodness = 0.0;
            for pattern in Correctness::patterns(){
                // considering a world where we did guess word and got pattern as the correctness.
                // Now compute what then is left
                let mut in_pattern_total = 0;
                for (candidate, count) in &*self.remaining {
                    let g = Guess {
                        word:Cow::Borrowed(word),
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
        best.unwrap().word.to_string()
    }
}

