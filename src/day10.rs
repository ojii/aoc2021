use crate::day10::Character::Curly;
use crate::utils::parse_lines;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Character {
    Paren,
    Bracket,
    Curly,
    Angle,
}

impl Character {
    fn syntax_error_score(&self) -> u32 {
        match self {
            Character::Paren => 3,
            Character::Bracket => 57,
            Character::Curly => 1197,
            Character::Angle => 25137,
        }
    }

    fn auto_correct_score(&self) -> u64 {
        match self {
            Character::Paren => 1,
            Character::Bracket => 2,
            Character::Curly => 3,
            Character::Angle => 4,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Open(Character),
    Close(Character),
}

impl Instruction {
    fn from_char(c: char) -> Result<Instruction, ()> {
        match c {
            '(' => Ok(Instruction::Open(Character::Paren)),
            '[' => Ok(Instruction::Open(Character::Bracket)),
            '{' => Ok(Instruction::Open(Character::Curly)),
            '<' => Ok(Instruction::Open(Character::Angle)),
            ')' => Ok(Instruction::Close(Character::Paren)),
            ']' => Ok(Instruction::Close(Character::Bracket)),
            '}' => Ok(Instruction::Close(Character::Curly)),
            '>' => Ok(Instruction::Close(Character::Angle)),
            other => {
                println!("Unexpected input: {}", other);
                Err(())
            }
        }
    }
}

#[derive(Debug)]
enum Line {
    Corrupt(String, Character),
    Incomplete(String, Vec<Character>),
    Valid(String),
}

impl Line {
    fn syntax_error_score(&self) -> Option<u32> {
        match self {
            Self::Corrupt(_, marker) => Some(marker.syntax_error_score()),
            _ => None,
        }
    }

    fn auto_correct_score(&self) -> Option<u64> {
        match self {
            Self::Incomplete(_, missing) => Some(missing.iter().rev().fold(0, |acc, character| {
                (acc * 5) + character.auto_correct_score()
            })),
            _ => None,
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(s.len() / 2);
        for instruction in s.chars().map(|c| Instruction::from_char(c)) {
            let instruction = instruction?;
            match instruction {
                Instruction::Open(character) => stack.push(character),
                Instruction::Close(character) => match stack.pop() {
                    Some(previous) if previous != character => {
                        return Ok(Line::Corrupt(s.to_owned(), character))
                    }
                    _ => (),
                },
            }
        }
        if stack.is_empty() {
            Ok(Line::Valid(s.to_owned()))
        } else {
            Ok(Line::Incomplete(s.to_owned(), stack))
        }
    }
}

#[derive(Debug)]
struct Navigation {
    lines: Vec<Line>,
}

impl Navigation {
    fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }
    fn syntax_error_score(&self) -> u32 {
        self.lines.iter().flat_map(Line::syntax_error_score).sum()
    }

    fn auto_correct_score(&self) -> u64 {
        let scores = self
            .lines
            .iter()
            .flat_map(Line::auto_correct_score)
            .sorted()
            .collect_vec();
        scores[scores.len() / 2]
    }
}

impl FromStr for Navigation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(parse_lines(s).collect()))
    }
}

pub fn run() {
    let navigation = Navigation::from_str(include_str!("data/10")).unwrap();
    println!("{}", navigation.syntax_error_score());
    println!("{}", navigation.auto_correct_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_error_score() {
        assert_eq!(
            Navigation::from_str(TEST_INPUT)
                .unwrap()
                .syntax_error_score(),
            26397
        )
    }

    #[test]
    fn test_autocorrect_score() {
        assert_eq!(
            Navigation::from_str(TEST_INPUT)
                .unwrap()
                .auto_correct_score(),
            288957
        )
    }
}
