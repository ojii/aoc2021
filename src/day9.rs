use crate::utils::{direct_neighbour_values, direct_neighbours};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Heightmap {
    points: HashMap<(usize, usize), u32>,
}

impl Heightmap {
    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.points.iter().flat_map(|(pos, height)| {
            if direct_neighbour_values(pos, &self.points)
                .all(|neighbour_height| neighbour_height > height)
            {
                Some(*pos)
            } else {
                None
            }
        })
    }

    fn basins(&self) -> impl Iterator<Item = HashSet<(usize, usize)>> + '_ {
        self.low_points().map(|pos| self.basin(&pos))
    }

    fn basin(&self, pos: &(usize, usize)) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::new();
        let mut search_queue = vec![*pos];
        while let Some(pos) = search_queue.pop() {
            for neighbour in direct_neighbours(&pos).flat_map(|pos| {
                self.points
                    .get(&pos)
                    .and_then(|height| if *height < 9 { Some(pos) } else { None })
            }) {
                if basin.insert(neighbour.clone()) {
                    search_queue.push(neighbour);
                }
            }
        }
        basin
    }

    fn three_largest_basin_size(&self) -> usize {
        self.basins()
            .map(|basin| basin.len())
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, size| acc * size)
    }

    fn risk_level(&self) -> u32 {
        self.low_points()
            .map(|pos| self.points.get(&pos).unwrap() + 1)
            .sum()
    }
}

impl FromStr for Heightmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Heightmap {
            points: HashMap::from_iter(s.lines().enumerate().flat_map(|(y, line)| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .enumerate()
                    .map(move |(x, height)| ((x, y), height))
            })),
        })
    }
}

pub fn run() {
    let heightmap = Heightmap::from_str(include_str!("data/9")).unwrap();
    println!("{}", heightmap.risk_level());
    println!("{}", heightmap.three_largest_basin_size());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_low_points() {
        let heightmap = Heightmap::from_str(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        )
        .unwrap();
        assert_eq!(heightmap.risk_level(), 15);
    }

    #[test]
    fn test_three_largest_basins() {
        let heightmap = Heightmap::from_str(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        )
        .unwrap();
        assert_eq!(heightmap.three_largest_basin_size(), 1134);
    }
}
