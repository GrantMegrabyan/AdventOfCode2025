use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let result = input
        .trim()
        .lines()
        .map(|line| {
            let chars = line.bytes().collect::<Vec<_>>();
            let mut p1 = 0;
            let mut p2 = 1;
            let mut p2_max = chars[p2];
            while p2 < line.len() {
                if p2 < line.len() - 1 && chars[p2] > chars[p1] {
                    p1 = p2;
                    p2_max = chars[p2 + 1];
                } else {
                    p2_max = p2_max.max(chars[p2]);
                }
                p2 += 1;
            }
            ((chars[p1] - b'0') * 10 + (p2_max - b'0')) as u64
        })
        .sum::<u64>();

    Ok(result)
}

pub fn part2(input: &str) -> Result<u64> {
    const SIZE: usize = 12;
    let result = input
        .trim()
        .lines()
        .map(|line| {
            let nums = line.bytes().map(|c| (c - b'0') as u64).collect::<Vec<_>>();

            let line_size = line.len();
            let mut prev_ptr = 0;
            let mut res = 0;
            for i in 0..SIZE {
                let search_space = &nums[prev_ptr..line_size - SIZE + i + 1];

                // Find the max value and its position
                let (max_ptr, &max_val) = search_space
                    .iter()
                    .enumerate()
                    // This makes sure we find the first occurance of the max value
                    .reduce(|(i, a), (j, b)| if b > a { (j, b) } else { (i, a) })
                    .unwrap();
                prev_ptr += max_ptr + 1;
                res = res * 10 + max_val;
            }

            res as u64
        })
        .sum::<u64>();

    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3121910778619);
    }
}
