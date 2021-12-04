use crate::utils::parse_lines;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MostCommonBit {
    Equal,
    One,
    Zero,
}

fn find_most_common_bits<const N: usize>(lines: &Vec<Line<N>>) -> [MostCommonBit; N] {
    let mut bits = [MostCommonBit::Equal; N];
    for (index, (zeros, ones)) in lines
        .iter()
        .fold([(0, 0); N], |mut zeros_and_ones, line| {
            for index in 0..N {
                if line.bits[index] {
                    zeros_and_ones[index].1 += 1;
                } else {
                    zeros_and_ones[index].0 += 1;
                }
            }
            zeros_and_ones
        })
        .iter()
        .enumerate()
    {
        if zeros > ones {
            bits[index] = MostCommonBit::Zero;
        } else if ones > zeros {
            bits[index] = MostCommonBit::One;
        }
    }
    bits
}

#[derive(Debug, Clone)]
struct Line<const N: usize> {
    bits: [bool; N],
}

impl<const N: usize> FromStr for Line<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = [false; N];
        for (index, char) in s.chars().enumerate() {
            bits[index] = char == '1';
        }
        Ok(Self { bits })
    }
}

#[derive(Debug)]
struct Data<const N: usize> {
    lines: Vec<Line<N>>,
    bits: [MostCommonBit; N],
}

impl<const N: usize> Data<N> {
    fn new(lines: Vec<Line<N>>) -> Self {
        let bits = find_most_common_bits(&lines);
        Self { lines, bits }
    }

    fn gamma(&self) -> usize {
        bits_to_int(self.bits.iter().map(|bit| {
            if *bit == MostCommonBit::Zero {
                false
            } else {
                true
            }
        }))
    }

    fn epsilon(&self) -> usize {
        bits_to_int(self.bits.iter().map(|bit| {
            if *bit == MostCommonBit::One {
                false
            } else {
                true
            }
        }))
    }

    fn power_consumption(&self) -> usize {
        self.gamma() * self.epsilon()
    }

    fn oxygen_generator_rating(&self) -> usize {
        let mut candidates = self.lines.clone();
        for index in 0..N {
            candidates = match find_most_common_bits(&candidates)[index] {
                MostCommonBit::One | MostCommonBit::Equal => {
                    filter_candidates(candidates, index, true)
                }
                MostCommonBit::Zero => filter_candidates(candidates, index, false),
            };
            if candidates.len() == 1 {
                return bits_to_int(candidates[0].bits);
            }
        }
        unreachable!()
    }

    fn co2_scrubber_rating(&self) -> usize {
        let mut candidates = self.lines.clone();
        for index in 0..N {
            candidates = match find_most_common_bits(&candidates)[index] {
                MostCommonBit::One | MostCommonBit::Equal => {
                    filter_candidates(candidates, index, false)
                }
                MostCommonBit::Zero => filter_candidates(candidates, index, true),
            };
            if candidates.len() == 1 {
                return bits_to_int(candidates[0].bits);
            }
        }
        unreachable!()
    }

    fn life_support_rating(&self) -> usize {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

fn filter_candidates<const N: usize>(
    candidates: Vec<Line<N>>,
    index: usize,
    flag: bool,
) -> Vec<Line<N>> {
    candidates
        .into_iter()
        .filter(|line| line.bits[index] == flag)
        .clone()
        .collect()
}

fn bits_to_int<I: IntoIterator<Item = bool>>(bits: I) -> usize {
    bits.into_iter()
        .fold(0, |result, bit| (result << 1) ^ (bit as usize))
}

pub fn run() {
    let input = include_str!("data/3");
    let data = Data::<12>::new(parse_lines(input).collect());
    println!("{}", data.power_consumption());
    println!("{}", data.life_support_rating());
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_bits_to_int() {
        assert_eq!(bits_to_int([true, false, true, true, false]), 22);
    }

    #[test]
    fn test_power_consumption() {
        assert_eq!(
            Data::<5>::new(parse_lines(TEST_INPUT).collect()).power_consumption(),
            198
        );
    }

    #[test]
    fn test_life_support_rating() {
        assert_eq!(
            Data::<5>::new(parse_lines(TEST_INPUT).collect()).life_support_rating(),
            230
        );
    }
}
