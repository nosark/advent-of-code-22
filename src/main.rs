use advent_of_code_22::day_one::{prepare_input, get_max_calories_by_elf, get_three_largest_calorie_amounts_by_elf, INPUT};
use std::time::Instant;

fn main() {
    let before = Instant::now();
    let result = prepare_input(INPUT);
    let score = get_max_calories_by_elf(result);
    println!("score: {:#?}", score.unwrap());
    println!("time elasped: {:.2?}", before.elapsed());    
}
