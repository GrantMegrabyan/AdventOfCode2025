use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let first_op_pos = input.find(|c| matches!(c, '*' | '+')).unwrap();
    let nums = input[..first_op_pos]
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let ops = input[first_op_pos..].split_whitespace().collect::<Vec<_>>();
    let chunks = nums.chunks(ops.len()).collect::<Vec<_>>();

    let mut results = Vec::<u64>::with_capacity(ops.len());
    for (i, &op) in ops.iter().enumerate() {
        let f = match op {
            "+" => |a: u64, b: u64| a + b,
            "*" => |a: u64, b: u64| a * b,
            _ => panic!(),
        };
        let mut acc = match op {
            "+" => 0,
            "*" => 1 as u64,
            _ => panic!(),
        };

        for ch in 0..chunks.len() {
            acc = f(acc, chunks[ch][i]);
        }
        results.push(acc);
    }
    Ok(results.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 4277556);
    }
}
