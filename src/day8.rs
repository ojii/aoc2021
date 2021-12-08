use crate::utils::parse_lines;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Wire {
    fn from(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("Unexpected wire {}", c),
        }
    }
}

#[derive(Debug)]
struct Entry {
    patterns: [HashSet<Wire>; 10],
    digits: [HashSet<Wire>; 4],
}

impl Entry {
    fn decoded_digits(&self) -> [u8; 4] {
        // find the one, four, seven and eight patterns.
        let (one, four, seven, eight) = self.patterns.iter().fold(
            (
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ),
            |(one, four, seven, eight), pat| match pat.len() {
                2 => (pat.clone(), four, seven, eight),
                3 => (one, four, pat.clone(), eight),
                4 => (one, pat.clone(), seven, eight),
                7 => (one, four, seven, pat.clone()),
                _ => (one, four, seven, eight),
            },
        );
        // find the candidates for two, three and five
        let two_three_or_five = self
            .patterns
            .iter()
            .filter(|pat| pat.len() == 5)
            .cloned()
            .collect_vec();
        // find the candidates for zero, six and nine
        let zero_six_or_nine = self
            .patterns
            .iter()
            .filter(|pat| pat.len() == 6)
            .cloned()
            .collect_vec();
        // find the "d" wire using the intersection of the two, three, five candidates
        // then intersecting that with the difference between four and one
        let d = two_three_or_five
            .iter()
            .cloned()
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap()
            .intersection(&four.difference(&one).cloned().collect())
            .cloned()
            .next()
            .unwrap();
        // find the "a" wire using the difference between seven and one
        let a = seven.difference(&one).next().unwrap().clone();
        // find the "b" wire from the difference between four and one that is not the "d" wire
        let b = four
            .difference(&one)
            .filter(|&wire| wire != &d)
            .next()
            .unwrap()
            .clone();
        // find zero from the candidates where the "d" wire is not set
        let zero = zero_six_or_nine
            .iter()
            .find(|pat| !pat.contains(&d))
            .unwrap()
            .clone();
        // find nine from the candidates by checking if it shares all the wires from one and "d"
        let nine = zero_six_or_nine
            .iter()
            .find(|&pat| one.iter().all(|wire| pat.contains(wire)) && pat.contains(&d))
            .unwrap()
            .clone();
        // six is the last candidate from that group
        let six = zero_six_or_nine
            .iter()
            .find(|&pat| pat != &zero && pat != &nine)
            .unwrap()
            .clone();
        // find three from candidates by checking if it shares all the wires from one
        let three = two_three_or_five
            .iter()
            .find(|&pat| one.iter().all(|wire| pat.contains(wire)))
            .unwrap()
            .clone();
        // find five from the candidates by checking if it contains the "b" wire.
        let five = two_three_or_five
            .iter()
            .find(|&pat| pat.contains(&b))
            .unwrap()
            .clone();
        // two is the remaining candidate
        let two = two_three_or_five
            .iter()
            .find(|&pat| pat != &three && pat != &five)
            .unwrap()
            .clone();
        let mut numbers = [0; 4];
        for (index, digit) in self.digits.iter().enumerate() {
            if digit == &zero {
                numbers[index] = 0;
            } else if digit == &one {
                numbers[index] = 1;
            } else if digit == &two {
                numbers[index] = 2;
            } else if digit == &three {
                numbers[index] = 3;
            } else if digit == &four {
                numbers[index] = 4;
            } else if digit == &five {
                numbers[index] = 5;
            } else if digit == &six {
                numbers[index] = 6;
            } else if digit == &seven {
                numbers[index] = 7;
            } else if digit == &eight {
                numbers[index] = 8;
            } else if digit == &nine {
                numbers[index] = 9;
            }
        }
        numbers
    }

    fn digit(&self) -> u32 {
        let decoded = self.decoded_digits();
        (decoded[0] as u32 * 1000)
            + (decoded[1] as u32 * 100)
            + (decoded[2] as u32 * 10)
            + (decoded[3] as u32)
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, digits) = s.split_once(" | ").ok_or(())?;
        Ok(Self {
            patterns: decode_wirings(patterns),
            digits: decode_wirings(digits),
        })
    }
}

fn decode_wirings<const N: usize>(s: &str) -> [HashSet<Wire>; N] {
    let mut wirings = [(); N].map(|_| HashSet::new());
    for (index, pattern) in s.split(" ").enumerate() {
        wirings[index] = pattern.chars().map(|c| Wire::from(c)).collect();
    }
    wirings
}

pub fn run() {
    let input = include_str!("data/8");
    println!(
        "{}",
        parse_lines::<Entry>(input)
            .flat_map(|entry| entry.decoded_digits())
            .filter(|&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
            .count()
    );
    println!(
        "{}",
        parse_lines::<Entry>(input)
            .map(|entry| entry.digit())
            .sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_digits() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(
            parse_lines::<Entry>(input)
                .flat_map(|entry| entry.decoded_digits())
                .filter(|&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
                .count(),
            26
        );
    }

    #[test]
    fn test_sum() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(
            parse_lines::<Entry>(input)
                .map(|entry| entry.digit())
                .sum::<u32>(),
            61229
        );
    }
}
