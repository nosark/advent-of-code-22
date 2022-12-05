use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_max_calories_by_elf(file_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let mut max_calories = i64::MIN;
    let mut current_sum: i64 = 0;
    // need to read and add each line until empty line
    // compare that number to current max
    // replace max or move on
    for line in reader.lines() {
        match line {
            Ok(val) => {
                if val.len() > 0 {
                    current_sum += val.parse::<i64>().unwrap();
                } else {
                    max_calories = i64::max(current_sum, max_calories);
                    current_sum = 0;
                }
            }
            Err(e) => {
                println!("Something went wrong: {}", e);
            }
        }
    }

    Ok(max_calories)
}

#[cfg(test)]
mod tests {
    use super::get_max_calories_by_elf;

    #[test]
    fn get_max_calories_simple() {
        let calories = get_max_calories_by_elf("res/tests/test1_day1.txt");

        assert_eq!(calories.unwrap(), 300)
    }
}
