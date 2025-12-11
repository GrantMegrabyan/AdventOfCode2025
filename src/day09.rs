use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let coords: Vec<(u64, u64)> = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let area =
                (coords[i].0.abs_diff(coords[j].0) + 1) * (coords[i].1.abs_diff(coords[j].1) + 1);
            max_area = max_area.max(area);
        }
    }
    Ok(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 50);
    }
}
