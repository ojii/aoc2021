use crate::utils::parse_lines;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Point = (u32, u32);

#[derive(Debug, Clone)]
enum Line {
    Straight { start: Point, end: Point },
    Diagonal { start: Point, end: Point },
}

impl Line {
    fn points(&self) -> Vec<Point> {
        match self {
            Line::Straight { start, end } => straight_line(start, end),
            Line::Diagonal { start, end } => diagonal_line(start, end),
        }
    }

    fn is_straight(&self) -> bool {
        match self {
            Line::Straight { start, end } => true,
            Line::Diagonal { start, end } => false,
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start = parse_coord(start);
        let end = parse_coord(end);
        if start.0 == end.0 || start.1 == end.1 {
            Ok(Line::Straight { start, end })
        } else {
            Ok(Line::Diagonal { start, end })
        }
    }
}

fn any_direction_range(a: u32, b: u32) -> impl Iterator<Item = u32> {
    if a > b {
        b..=a
    } else {
        a..=b
    }
}

fn straight_line(start: &Point, end: &Point) -> Vec<Point> {
    if start.0 == end.0 {
        any_direction_range(start.1, end.1)
            .map(|y| (start.0, y))
            .collect()
    } else {
        any_direction_range(start.0, end.0)
            .map(|x| (x, start.1))
            .collect()
    }
}

fn diagonal_line(start: &Point, end: &Point) -> Vec<Point> {
    let (start, end) = if start.0 > end.0 {
        (end, start)
    } else {
        (start, end)
    };
    if start.1 > end.1 {
        (start.0..=end.0).zip((end.1..=start.1).rev()).collect()
    } else {
        (start.0..=end.0).zip(start.1..=end.1).collect()
    }
}

fn parse_coord(s: &str) -> (u32, u32) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn danger_points(lines: &[Line]) -> HashMap<(u32, u32), u32> {
    let mut points = HashMap::new();
    for line in lines {
        for point in line.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }
    points
}

fn straights(lines: &[Line]) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| line.is_straight())
        .cloned()
        .collect_vec()
}

pub fn run() {
    let lines = parse_lines::<Line>(include_str!("data/5")).collect_vec();
    let points = danger_points(&straights(&lines));
    println!("{}", points.values().filter(|&&n| n >= 2).count());
    let points = danger_points(&lines);
    println!("{}", points.values().filter(|&&n| n >= 2).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_danger_points_straight() {
        let lines = parse_lines::<Line>(TEST_INPUT).collect_vec();
        assert_eq!(
            danger_points(&straights(&lines))
                .values()
                .filter(|&&n| n >= 2)
                .count(),
            5
        );
    }

    #[test]
    fn test_danger_points_all() {
        let lines = parse_lines::<Line>(TEST_INPUT).collect_vec();
        assert_eq!(
            danger_points(&lines).values().filter(|&&n| n >= 2).count(),
            12
        );
    }
}
