use advent_of_code_22::day_one::{
    get_max_calories_by_elf, get_three_largest_calorie_amounts_by_elf, prepare_input, INPUT,
};
use std::time::Instant;

fn main() {
    let before = Instant::now();
    let input = prepare_input(INPUT);

    let top_three_results = get_three_largest_calorie_amounts_by_elf(input).unwrap();
    let sum = top_three_results[0] + top_three_results[1] + top_three_results[2];
    println!("The top three elfs were carrying: {:#?}", sum);
    println!("time elasped: {:.2?}", before.elapsed());
}
