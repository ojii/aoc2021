use std::str::FromStr;

pub fn parse_lines<'a, F: 'a + FromStr>(s: &'a str) -> impl Iterator<Item = F> + 'a {
    s.lines().flat_map(|l| l.parse::<F>())
}
