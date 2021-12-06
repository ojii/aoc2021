use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct Population {
    fish: [u64; 9],
}

impl Population {
    fn new(one: u64, two: u64, three: u64, four: u64, five: u64, six: u64) -> Self {
        Self {
            fish: [0, one, two, three, four, five, six, 0, 0],
        }
    }

    fn advance(&mut self) {
        let zero = self.fish[0];
        self.fish.rotate_left(1);
        self.fish[6] += zero;
    }

    fn count(&self) -> u64 {
        self.fish.iter().sum()
    }
}

impl FromStr for Population {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two, three, four, five, six) = s.chars().fold(
            (0, 0, 0, 0, 0, 0),
            |(one, two, three, four, five, six), c| match c {
                '1' => (one + 1, two, three, four, five, six),
                '2' => (one, two + 1, three, four, five, six),
                '3' => (one, two, three + 1, four, five, six),
                '4' => (one, two, three, four + 1, five, six),
                '5' => (one, two, three, four, five + 1, six),
                '6' => (one, two, three, four, five, six + 1),
                _ => (one, two, three, four, five, six),
            },
        );
        Ok(Self::new(one, two, three, four, five, six))
    }
}

pub fn run() {
    let mut population = Population::from_str(include_str!("data/6")).unwrap();
    for _ in 0..80 {
        population.advance();
    }
    println!("{}", population.count());
    for _ in 0..176 {
        population.advance();
    }
    println!("{}", population.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population() {
        let mut population = Population::from_str("3,4,3,1,2").unwrap();
        for _ in 0..80 {
            population.advance();
        }
        assert_eq!(population.count(), 5934);
        for _ in 0..176 {
            population.advance();
        }
        assert_eq!(population.count(), 26984457539);
    }
}
