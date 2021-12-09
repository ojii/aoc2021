use num_traits::{CheckedAdd, CheckedSub, One};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

pub fn parse_lines<'a, F: 'a + FromStr>(s: &'a str) -> impl Iterator<Item = F> + 'a {
    s.lines().flat_map(|l| l.parse::<F>())
}

#[derive(Debug)]
enum NeighbourDirection {
    Top,
    Right,
    Down,
    Left,
}

impl NeighbourDirection {
    fn apply<I: CheckedAdd + CheckedSub + One + Clone>(&self, position: &(I, I)) -> Option<(I, I)> {
        let (x, y) = position.clone();
        match self {
            NeighbourDirection::Top => y.checked_sub(&I::one()).map(|y| (x, y)),
            NeighbourDirection::Right => x.checked_add(&I::one()).map(|x| (x, y)),
            NeighbourDirection::Down => y.checked_add(&I::one()).map(|y| (x, y)),
            NeighbourDirection::Left => x.checked_sub(&I::one()).map(|x| (x, y)),
        }
    }
}

static DIRECT_NEIGHBOURS: [NeighbourDirection; 4] = [
    NeighbourDirection::Top,
    NeighbourDirection::Right,
    NeighbourDirection::Down,
    NeighbourDirection::Left,
];

pub fn direct_neighbours<K: One + CheckedSub + CheckedAdd + Clone>(
    position: &(K, K),
) -> impl Iterator<Item = (K, K)> + '_ {
    DIRECT_NEIGHBOURS
        .iter()
        .flat_map(|direction| direction.apply(position))
}

pub fn direct_neighbour_values<'a, K: Eq + Hash + One + CheckedSub + CheckedAdd + Clone, V>(
    position: &'a (K, K),
    points: &'a HashMap<(K, K), V>,
) -> impl Iterator<Item = &'a V> {
    direct_neighbours(position).flat_map(|pos| points.get(&pos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::collections::HashSet;

    #[rstest]
    #[case::zero_zero((0,0), vec!['b', 'd'])]
    #[case::one_zero((1,0), vec!['a', 'c', 'e'])]
    #[case::two_zero((2,0), vec!['b', 'f'])]
    #[case::zero_one((0,1), vec!['a', 'e', 'g'])]
    #[case::one_one((1,1), vec!['b', 'd', 'f', 'h'])]
    #[case::two_one((2,1), vec!['c', 'e', 'i'])]
    #[case::zero_two((0,2), vec!['d', 'h'])]
    #[case::one_two((1,2), vec!['e', 'g', 'i'])]
    #[case::two_two((2,2), vec!['f', 'h'])]
    fn test_direct_neighbour_values(#[case] pos: (usize, usize), #[case] expected: Vec<char>) {
        /*
            a b c
            d e f
            g h i
        */
        let points: HashMap<(usize, usize), char> = HashMap::from([
            ((0, 0), 'a'),
            ((1, 0), 'b'),
            ((2, 0), 'c'),
            ((0, 1), 'd'),
            ((1, 1), 'e'),
            ((2, 1), 'f'),
            ((0, 2), 'g'),
            ((1, 2), 'h'),
            ((2, 2), 'i'),
        ]);
        let neighbours: HashSet<char> = direct_neighbour_values(&pos, &points).cloned().collect();
        assert_eq!(neighbours, HashSet::from_iter(expected.into_iter()));
    }
}
