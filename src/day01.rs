use anyhow::Result;

const INIT: i32 = 50;

pub fn part1(input: &str) -> Result<i32> {
    let result = input
        .lines()
        .map(|line| (&line[..1], line[1..].parse::<i32>().unwrap() % 100))
        .fold((INIT, 0), |(acc, count), (dir, dist)| {
            let next = if dir == "L" {
                (acc - dist).max((acc - dist + 100) % 100)
            } else {
                (acc + dist) % 100
            };
            (next, count + if next == 0 { 1 } else { 0 })
        });
    Ok(result.1)
}

pub fn part2(input: &str) -> Result<i32> {
    let result = input
        .lines()
        .map(|line| (&line[..1], line[1..].parse::<i32>().unwrap()))
        .fold((INIT, 0), |(acc, count), (dir, dist)| {
            let new_value = if dir == "L" { acc - dist } else { acc + dist };
            let mut next = new_value % 100;
            if next < 0 {
                next = next + 100;
            }
            let crossed_zero = if new_value == 0 { 1 } else { 0 }
                + if acc * new_value < 0 { 1 } else { 0 }
                + (new_value / 100).abs();
            (next, count + crossed_zero)
        });
    Ok(result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(3, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(6, result);
    }
}
