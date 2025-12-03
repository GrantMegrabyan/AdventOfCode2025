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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = part1(input).unwrap();
        assert_eq!(3, result);
    }
}
