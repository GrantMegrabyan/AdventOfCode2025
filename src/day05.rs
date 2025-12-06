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

pub fn part2(input: &str) -> Result<u64> {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| {
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|(start, _)| *start);
    let valid_ids: u64 = ranges
        .iter()
        .fold(vec![], |mut acc: Vec<(u64, u64)>, (start, end)| {
            if let Some(last) = acc.last_mut()
                && last.1 >= *start
            {
                last.1 = last.1.max(*end);
            } else {
                acc.push((*start, *end));
            }
            acc
        })
        .iter()
        .map(|(start, end)| *end - *start + 1)
        .sum();

    Ok(valid_ids)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 14);
    }
}
