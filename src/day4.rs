use itertools::Itertools;
use std::str::FromStr;

fn matches(candidates: &[u32], drawn: &[&u32]) -> bool {
    candidates.iter().filter(|n| drawn.contains(n)).count() == 5
}

#[derive(Debug, Clone)]
struct Board {
    nums: [[u32; 5]; 5],
}

impl Board {
    fn check_win(&self, drawn: &[&u32]) -> bool {
        for row in &self.nums {
            if matches(row, drawn) {
                return true;
            }
        }
        for index in 0..5 {
            if matches(&self.nums.map(|row| row[index]), drawn) {
                return true;
            }
        }
        false
    }

    fn calculate_score(&self, drawn: &[&u32]) -> u32 {
        let unmarked = self
            .nums
            .iter()
            .flatten()
            .filter(|n| !drawn.contains(n))
            .cloned()
            .sum::<u32>();
        let last = **drawn.last().unwrap();
        unmarked * last
    }
}

#[derive(Debug)]
struct Bingo {
    order: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn find_first_winner(&self) -> u32 {
        for index in 5..self.order.len() {
            let drawn = self.order.iter().take(index).collect_vec();
            for board in &self.boards {
                if board.check_win(&drawn) {
                    return board.calculate_score(&drawn);
                }
            }
        }
        unreachable!()
    }

    fn find_last_winner(&self) -> u32 {
        let mut boards = self.boards.clone();
        for index in 5..self.order.len() {
            let drawn = self.order.iter().take(index).collect_vec();
            if boards.len() == 1 {
                let board = &boards[0];
                if board.check_win(&drawn) {
                    return board.calculate_score(&drawn);
                }
            } else {
                boards = boards
                    .into_iter()
                    .filter(|board| !board.check_win(&drawn))
                    .collect();
            }
        }
        unreachable!()
    }
}

impl FromStr for Bingo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let order = lines
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect_vec();
        let mut boards = Vec::new();
        while lines.peek().is_some() {
            lines.next();
            let mut nums = [[0u32; 5]; 5];
            for row_index in 0..5 {
                for (col_index, num) in lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .enumerate()
                {
                    nums[row_index][col_index] = num;
                }
            }
            boards.push(Board { nums })
        }
        Ok(Self { order, boards })
    }
}

pub fn run() {
    let bingo = Bingo::from_str(include_str!("data/4")).unwrap();
    println!("{}", bingo.find_first_winner());
    println!("{}", bingo.find_last_winner());
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_find_first_winner() {
        assert_eq!(
            Bingo::from_str(TEST_INPUT).unwrap().find_first_winner(),
            4512
        )
    }

    #[test]
    fn find_last_winner() {
        assert_eq!(
            Bingo::from_str(TEST_INPUT).unwrap().find_last_winner(),
            1924
        )
    }
}
