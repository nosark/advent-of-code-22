use advent_of_code_22::day_two::{get_score, prepare_input, INPUT_DAY_TWO};
use std::time::Instant;

fn main() {
    let before = Instant::now();
    let result = prepare_input(INPUT_DAY_TWO);
    let score = get_score(result);
    println!("length: {:#?}", score);
    println!("time elasped: {:.2?}", before.elapsed());
}
