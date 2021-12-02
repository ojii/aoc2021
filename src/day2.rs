use crate::utils::parse_lines;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ")
            .and_then(|(name, amount)| amount.parse::<i32>().ok().map(|num| (name, num)))
            .and_then(|(name, num)| match name {
                "forward" => Some(Command::Forward(num)),
                "up" => Some(Command::Up(num)),
                "down" => Some(Command::Down(num)),
                _ => None,
            })
            .ok_or(())
    }
}

#[derive(Debug, Default, PartialEq)]
struct SimpleSubmarine {
    distance: i32,
    depth: i32,
    aim: i32,
}

impl SimpleSubmarine {
    pub fn new(distance: i32, depth: i32, aim: i32) -> Self {
        Self {
            distance,
            depth,
            aim,
        }
    }
    pub fn navigate(&self, command: &Command, simple: bool) -> Self {
        match command {
            Command::Forward(num) => {
                if simple {
                    Self::new(self.distance + num, self.depth, self.aim)
                } else {
                    Self::new(self.distance + num, self.depth + (num * self.aim), self.aim)
                }
            }
            Command::Up(num) => {
                if simple {
                    Self::new(self.distance, self.depth - num, self.aim)
                } else {
                    Self::new(self.distance, self.depth, self.aim - num)
                }
            }
            Command::Down(num) => {
                if simple {
                    Self::new(self.distance, self.depth + num, self.aim)
                } else {
                    Self::new(self.distance, self.depth, self.aim + num)
                }
            }
        }
    }

    pub fn position(&self) -> i32 {
        self.distance * self.depth
    }
}

fn navigate(input: &str, simple: bool) -> SimpleSubmarine {
    parse_lines::<Command>(input).fold(SimpleSubmarine::default(), |sub, cmd| {
        sub.navigate(&cmd, simple)
    })
}

static INPUT: &'static str = include_str!("data/2");

pub fn run() {
    println!("{}", navigate(INPUT, true).position());
    println!("{}", navigate(INPUT, false).position());
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_simple() {
        assert_eq!(navigate(TEST_INPUT, true).position(), 150);
    }

    #[test]
    fn test_aimed() {
        assert_eq!(navigate(TEST_INPUT, false).position(), 900);
    }
}
