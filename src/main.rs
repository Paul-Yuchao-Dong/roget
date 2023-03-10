// use std::str::FromStr;
use clap::{Parser, clap_derive::ArgEnum};
use roget::{Wordle, Guesser};
const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[allow(non_snake_case)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, arg_enum)]
   Implementation: Implementation,
   
   #[clap(short, long)]
    max: usize,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
enum Implementation {
    Naive,
    Allocs,
    VecRem,
    Once
}

fn main() {
    let args = Args::parse();
    

    match args.Implementation {
        Implementation::Naive => play(roget::algorithms::Naive::new, Some(args.max)),
        Implementation::Allocs=> play(roget::algorithms::Allocs::new, Some(args.max)),
        Implementation::VecRem => play(roget::algorithms::VecRem::new, Some(args.max)),
        Implementation::Once => play(roget::algorithms::Once::new, Some(args.max)),

    }

}

fn play<G>(mut mk: impl FnMut()->G, max:Option<usize>) where G: Guesser {
    let w = Wordle::new();
    for answer in GAMES
            .split_whitespace()
            .take(max.unwrap_or(usize::MAX))
    {
        let answer_b: roget::Word = answer.as_bytes().try_into().unwrap();
        let guesser = (mk)();
        if let Some(score) = w.play(answer_b, guesser){
            println!("Guessed {} in {}", answer, score);
        }else{
            eprintln!("failed to guess ");
        };
    }
}