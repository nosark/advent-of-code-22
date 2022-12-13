pub const INPUT_DAY_TWO: &str = include_str!("../res/day_two_input.txt");

// Comments:
// X = 1 = Rock
// Y = 2 = Paper
// Z = 3 = Scissors
//
// A = Rock
// B = Paper
// C = Scissors
//
// A X = 3 + 1
// A Y = 6 + 2
// A Z =  0 + 3
//
// B X = 0 + 1
// B Y = 3 + 2
// B Z = 6 + 3
//
// C X = 6 + 1
// C Y = 0 + 2
// C Z = 3 + 3
//

/// Represents hand shapes thrown via Rock Paper Scissors
#[derive(Clone, Copy)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

/// We implement this conversion for later score calculation.
impl From<HandShape> for u32 {
    fn from(shape: HandShape) -> u32 {
        match shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

/// We implement the from trait to convert our input data into HandShapes
impl From<&str> for HandShape {
    fn from(character: &str) -> HandShape {
        match character {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissors,
            _ => panic!("unable to convert character to HandShape"),
        }
    }
}

impl From<HandShape> for Outcome {
    fn from(shape: HandShape) -> Outcome {
        match shape {
            HandShape::Rock => Outcome::Loss,
            HandShape::Paper => Outcome::Draw,
            HandShape::Scissors => Outcome::Win,
        }
    }
}

impl TryFrom<(HandShape, HandShape)> for Outcome {
    type Error = ();
    fn try_from(value: (HandShape, HandShape)) -> Result<Self, Self::Error> {
        match value {
            (HandShape::Rock, HandShape::Rock) => Ok(Outcome::Draw),
            (HandShape::Rock, HandShape::Paper) => Ok(Outcome::Win),
            (HandShape::Rock, HandShape::Scissors) => Ok(Outcome::Loss),
            (HandShape::Paper, HandShape::Rock) => Ok(Outcome::Loss),
            (HandShape::Paper, HandShape::Paper) => Ok(Outcome::Draw),
            (HandShape::Paper, HandShape::Scissors) => Ok(Outcome::Win),
            (HandShape::Scissors, HandShape::Rock) => Ok(Outcome::Win),
            (HandShape::Scissors, HandShape::Paper) => Ok(Outcome::Loss),
            (HandShape::Scissors, HandShape::Scissors) => Ok(Outcome::Draw),
        }
    }
}

impl TryFrom<(HandShape, Outcome)> for HandShape {
    type Error = ();
    fn try_from(value: (HandShape, Outcome)) -> Result<Self, Self::Error> {
        let result = match value {
            (HandShape::Rock, Outcome::Loss) => HandShape::Scissors,
            (HandShape::Rock, Outcome::Draw) => HandShape::Rock,
            (HandShape::Rock, Outcome::Win) => HandShape::Paper,
            (HandShape::Paper, Outcome::Loss) => HandShape::Rock,
            (HandShape::Paper, Outcome::Draw) => HandShape::Paper,
            (HandShape::Paper, Outcome::Win) => HandShape::Scissors,
            (HandShape::Scissors, Outcome::Loss) => HandShape::Paper,
            (HandShape::Scissors, Outcome::Draw) => HandShape::Scissors,
            (HandShape::Scissors, Outcome::Win) => HandShape::Rock,
        };

        Ok(result)
    }
}

#[derive(Clone, Copy)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl From<Outcome> for u32 {
    fn from(outcome: Outcome) -> u32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

pub fn prepare_input(input: &str) -> Vec<(HandShape, HandShape)> {
    input
        .trim()
        .split("\n")
        .map(convert_to_shapes)
        .collect::<Vec<(HandShape, HandShape)>>()
}

fn convert_to_shapes(input: &str) -> (HandShape, HandShape) {
    let mut chars_split = input.split_whitespace();
    (
        HandShape::from(chars_split.next().unwrap()),
        HandShape::from(chars_split.next().unwrap()),
    )
}

pub fn get_score(input: Vec<(HandShape, HandShape)>) -> Result<u32, std::num::ParseIntError> {
    let mut score = 0;
    for round in input {
        let match_outcome = Outcome::try_from(round);
        let current_score: u32 = u32::from(match_outcome.unwrap()) + u32::from(round.1);
        score += current_score;
    }

    Ok(score)
}

pub fn get_score_with_new_rules(
    input: Vec<(HandShape, HandShape)>,
) -> Result<u32, std::num::IntErrorKind> {
    let mut score: u32 = 0;
    for round in input {
        let encrypted_outcome = Outcome::from(round.1);
        let hand_to_throw = HandShape::try_from((round.0, encrypted_outcome));
        score += u32::from(hand_to_throw.unwrap()) + u32::from(encrypted_outcome);
    }

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::{get_score, get_score_with_new_rules, prepare_input, INPUT_DAY_TWO};

    #[test]
    fn solve_part_one() {
        let input = prepare_input(INPUT_DAY_TWO);
        let result = get_score(input).unwrap();

        assert_eq!(result, 14375);
    }

    #[test]
    fn solve_part_two() {
        let input = prepare_input(INPUT_DAY_TWO);
        let result = get_score_with_new_rules(input).unwrap();

        assert_eq!(result, 10274);
    }
}
