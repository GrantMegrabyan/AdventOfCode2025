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
}
