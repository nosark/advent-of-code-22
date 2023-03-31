// Load our input into a string.
pub const INPUT: &str = include_str!("../res/day_one_input.txt");

pub fn prepare_input(input: &str) -> Vec<u32> {
    let os_type = std::env::consts::OS;
    let mut split_token = " ";
    match os_type {
        "windows" => split_token = "\r\n",
        "macos" => split_token = "\n\n",
        _ => println!("something went wrong in determining the operating system"),
    }

    input
        .trim()
        .split(split_token)
        .map(try_parse_and_sum_lines)
        .collect::<Result<_, _>>()
        .expect("failed to parse numbers")
}

pub fn try_parse_and_sum_lines(input: &str) -> Result<u32, std::num::ParseIntError> {
    let mut sum = 0;
    for line in input.lines() {
        sum += line.parse::<u32>()?;
    }

    Ok(sum)
}

pub fn get_max_calories_by_elf(input: Vec<u32>) -> Result<u32, Box<dyn std::error::Error>> {
    let max = input.into_iter().max();
    Ok(max.unwrap())
}

pub fn get_three_largest_calorie_amounts_by_elf(
    input: Vec<u32>,
) -> Result<[u32; 3], Box<dyn std::error::Error>> {
    let mut top_three_arr: [u32; 3] = [u32::MIN; 3];
    for num in input {
        top_three_arr = *insert_and_shift_arr_elements(&mut top_three_arr, num);
    }

    Ok(top_three_arr)
}

pub fn insert_and_shift_arr_elements(arr: &mut [u32; 3], num: u32) -> &[u32; 3] {
    if num > arr[2] {
        arr[0] = arr[1];
        arr[1] = arr[2];
        arr[2] = num;
    } else if num > arr[1] && num != arr[2] {
        arr[0] = arr[1];
        arr[1] = num;
    } else if num > arr[0] && num != arr[1] {
        arr[0] = num;
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::{
        get_max_calories_by_elf, get_three_largest_calorie_amounts_by_elf, prepare_input, INPUT,
    };
    #[test]
    fn day_one_part_1_solve() {
        let input = prepare_input(INPUT);
        let calories = get_max_calories_by_elf(input).unwrap();
        assert_eq!(calories, 72240)
    }

    #[test]
    fn day_one_part2_solve() {
        let input = prepare_input(INPUT);
        let top_three = get_three_largest_calorie_amounts_by_elf(input).unwrap();
        let sum = top_three[0] + top_three[1] + top_three[2];
        assert_eq!(sum, 210957)
    }

    #[test]
    fn get_max_calories_simple() {
        let test_input = include_str!("../res/tests/test1_day1.txt");
        let input = prepare_input(test_input);
        let calories = get_max_calories_by_elf(input);

        assert_eq!(calories.unwrap(), 300)
    }

    #[test]
    fn get_top_three_elf_calorie_totals() {
        let test_input = include_str!("../res/tests/test2_day1.txt");
        let input = prepare_input(test_input);
        let results = get_three_largest_calorie_amounts_by_elf(input).unwrap();

        assert_eq!(results, [15, 20, 30])
    }

    #[test]
    fn get_top_three_totals_all_pos() {
        let test_input = include_str!("../res/tests/test3_day1.txt");
        let input = prepare_input(test_input);
        let results = get_three_largest_calorie_amounts_by_elf(input).unwrap();

        assert_eq!(results, [4, 5, 6])
    }

    #[test]
    fn get_top_three_clustered_no_singles() {
        let test_input = include_str!("../res/tests/test4_day1.txt");
        let input = prepare_input(test_input);
        let results = get_three_largest_calorie_amounts_by_elf(input).unwrap();

        assert_eq!(results, [1200, 2700, 4000])
    }
}
