use std::collections::HashSet;

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
            match ch.is_ascii_lowercase() {
                true => score = ch as u32 - 96,
                false => score = ch as u32 - 38,
            }
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::get_item_priority_score;
    use super::prepare_input;

    const SAMPLE_TEST_CASE: &str = include_str!("../res/tests/day3_sample_test.txt");

    #[test]
    fn day_three_sample_input_test() {
        let input = prepare_input(SAMPLE_TEST_CASE);
        let result = get_item_priority_score(input).unwrap();

        assert_eq!(result, 157)
    }
}
