use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| {
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    let fresh_ids = ids
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|(start, end)| id >= start && id <= end))
        .collect::<Vec<_>>();

    Ok(fresh_ids.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 3);
    }
}
