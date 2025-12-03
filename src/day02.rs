use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let result = input
        .trim()
        .split(",")
        .into_iter()
        .map(|s| s.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .fold(0, |acc, (start, end)| {
            let mut sum = 0;
            for num in start..=end {
                let str = format!("{}", num);
                if str[..str.len() / 2] == str[str.len() / 2..] {
                    sum += num;
                }
            }
            acc + sum
        });
    Ok(result)
}

pub fn part2(input: &str) -> Result<u64> {
    let result = input
        .trim()
        .split(",")
        .into_iter()
        .map(|s| s.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .fold(0, |acc, (start, end)| {
            let mut sum = 0;
            for num in start..=end {
                let str = format!("{}", num);
                for window_size in 1..=str.len() / 2 {
                    // Make sure we can fit full size windows
                    if str.len() % window_size != 0 {
                        continue;
                    }

                    let windows_cnt = str.len() / window_size;
                    let mut windows = (0..windows_cnt)
                        .map(|i| (i * window_size, (i + 1) * window_size))
                        .map(|(s, e)| &str[s..e]);

                    let first = &str[..window_size];
                    if windows.all(|x| x == first) {
                        sum += num;
                        break;
                    }
                }
            }
            acc + sum
        });
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 4174379265);
    }
}
