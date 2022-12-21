use advent_of_code_22::day_three::{get_item_priority_score, prepare_input, DAY_THREE_INPUT};
use std::time::Instant;

fn main() {
    let before = Instant::now();
    let result = prepare_input(DAY_THREE_INPUT);
    let score = get_item_priority_score(result);
    println!("score: {:#?}", score.unwrap());
    println!("time elasped: {:.2?}", before.elapsed());
}
