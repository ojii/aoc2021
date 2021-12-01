use crate::utils::parse_lines;
use itertools::Itertools;

fn count_increases(input: &str) -> usize {
    parse_lines::<i32>(input)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn count_triple_increases(input: &str) -> usize {
    parse_lines::<i32>(input)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn run() {
    let input = include_str!("data/1");
    println!("{}", count_increases(input));
    println!("{}", count_triple_increases(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(TEST_INPUT), 7);
    }

    #[test]
    fn test_count_triple_increases() {
        assert_eq!(count_triple_increases(TEST_INPUT), 5);
    }
}
