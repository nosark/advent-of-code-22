use std::collections::{HashMap, HashSet};

/// Need to read each line and split line into two strings ( compartments )
/// Then, we compare each string to identify the item that is duplicated in both
/// Then we take the priority value of the duplicate item and add it to our sum:w

pub const DAY_THREE_INPUT: &'static str = include_str!("../res/day_three_input.txt");

pub fn prepare_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(split_string_into_compartments)
        .collect::<Vec<(&str, &str)>>()
}

fn get_badge_score(value: u8) -> u32 {
    let score;
    match value.is_ascii_lowercase() {
        true => score = value as u32 - 96,
        false => score = value as u32 - 38,
    }
    score
}

fn split_string_into_compartments(value: &str) -> (&str, &str) {
    let middle = value.len() / 2;
    let (first, second) = value.split_at(middle);
    (first, second)
}

pub fn get_item_priority_score(
    input: Vec<(&str, &str)>,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut score = 0;
    for compartments in input {
        score += find_unique_duplicate(compartments);
    }

    Ok(score)
}

fn find_unique_duplicate(value: (&str, &str)) -> u32 {
    let mut char_set = HashSet::<u8>::new();
    let mut score = 0;
    for c in value.0.bytes() {
        char_set.insert(c);
    }

    for ch in value.1.bytes() {
        if char_set.contains(&ch) {
            score = get_badge_score(ch);
            break;
        }
    }
    score
}

fn find_unique_duplicate_for_three(value: (&str, &str, &str)) -> u32 {
    let mut char_set = HashSet::<u8>::new();
    let mut score = 0;
    let mut possible_uniques = Vec::<char>::new();

    // need to get possible duplicates from first two strings
    // take a vec of those possible duplicates and search third string

    for c in value.0.bytes() {
        char_set.insert(c);
    }

    for ch in value.1.bytes() {
        if char_set.contains(&ch) {
            possible_uniques.push(ch as char);
        }
    }

    for letter in possible_uniques {
        if value.2.contains(letter) {
            score = get_badge_score(letter as u8);
        }
    }
    score
}

fn get_item_priority_score_group_threes(input: &str) -> u32 {
    let mut score: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    loop {
        score += find_unique_duplicate_for_three((
            lines[i as usize],
            lines[i + 1 as usize],
            lines[i + 2 as usize],
        ));
        if i >= lines.len() - 3 {
            break;
        } else {
            i += 3;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::day_three::get_item_priority_score_group_threes;

    use super::get_item_priority_score;
    use super::prepare_input;

    const SAMPLE_TEST_CASE: &str = include_str!("../res/tests/day3_sample_test.txt");
    const INPUT: &str = include_str!("../res/day_three_input.txt");
    #[test]
    fn day_three_sample_input_test() {
        let input = prepare_input(SAMPLE_TEST_CASE);
        let result = get_item_priority_score(input).unwrap();

        assert_eq!(result, 157)
    }

    #[test]
    fn day_three_solve() {
        let input = prepare_input(INPUT);
        let result = get_item_priority_score(input).unwrap();

        assert_eq!(result, 7824)
    }

    #[test]
    fn day_three_part_2_solve() {
        let result = get_item_priority_score_group_threes(INPUT);
        assert_eq!(result, 2798)
    }

    #[test]
    fn day_three_sample_test_two() {
        let result = get_item_priority_score_group_threes(SAMPLE_TEST_CASE);
        assert_eq!(result, 70)
    }
}
