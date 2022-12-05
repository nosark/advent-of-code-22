use advent_of_code_22::day_one::get_max_calories_by_elf;

fn main() {
    let calories = get_max_calories_by_elf("res/day_one.txt");
    println!(
        "The elf carrying the most calories is carrying: {} in total.",
        calories.unwrap()
    );
}
