#![feature(int_abs_diff)]
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn abs_diff(lhs: u32, rhs: u32) -> u32 {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

fn to_fuel_cost(diff: u32) -> u32 {
    (diff * (1 + diff)) / 2
}

#[derive(Debug)]
struct Crabs {
    positions: HashMap<u32, u32>,
    min: u32,
    max: u32,
}

impl Crabs {
    fn new(positions: HashMap<u32, u32>) -> Self {
        let (min, max) = &positions.keys().minmax().into_option().unwrap();
        let min = **min;
        let max = **max;
        Self {
            positions,
            min,
            max,
        }
    }

    fn calculate_simple_cost(&self, to: u32) -> u32 {
        self.positions
            .iter()
            .map(|(&position, &count)| abs_diff(position, to) * count)
            .sum()
    }

    fn calculate_real_cost(&self, to: u32) -> u32 {
        self.positions
            .iter()
            .map(|(&position, &count)| to_fuel_cost(abs_diff(position, to)) * count)
            .sum()
    }
}

impl FromStr for Crabs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = HashMap::new();
        for num in s.split(',').map(|n| n.parse().unwrap()) {
            *positions.entry(num).or_insert(0) += 1;
        }
        Ok(Self::new(positions))
    }
}

fn ideal_simple_fuel_consumption(crabs: &Crabs) -> u32 {
    (crabs.min..=crabs.max)
        .map(|position| crabs.calculate_simple_cost(position))
        .min()
        .unwrap()
}

fn ideal_real_fuel_consumption(crabs: &Crabs) -> u32 {
    (crabs.min..=crabs.max)
        .map(|position| crabs.calculate_real_cost(position))
        .min()
        .unwrap()
}

pub fn run() {
    let crabs = Crabs::from_str(include_str!("data/7")).unwrap();
    println!("{}", ideal_simple_fuel_consumption(&crabs));
    println!("{}", ideal_real_fuel_consumption(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_ideal_simple_fuel_consumption() {
        let crabs = Crabs::from_str(TEST_INPUT).unwrap();
        assert_eq!(ideal_simple_fuel_consumption(&crabs), 37)
    }
    #[test]
    fn test_ideal_real_fuel_consumption() {
        let crabs = Crabs::from_str(TEST_INPUT).unwrap();
        assert_eq!(ideal_real_fuel_consumption(&crabs), 168)
    }
}
