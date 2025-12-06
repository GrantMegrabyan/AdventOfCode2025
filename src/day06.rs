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

pub fn part2(input: &str) -> Result<u64> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = lines.pop().unwrap().bytes();
    let chars = lines
        .iter()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut op = '+';
    let mut acc = 0;
    let mut total = 0;

    fn read_num(rows: &Vec<Vec<u8>>, idx: usize) -> Option<u64> {
        let mut num = None;
        for i in 0..rows.len() {
            if idx < rows[i].len() && rows[i][idx] >= b'0' {
                num = Some(num.unwrap_or(0) * 10 + u64::from(rows[i][idx] - b'0'));
            }
        }
        num
    }

    for (idx, ch) in ops.enumerate() {
        match ch {
            b'+' => {
                total += acc;
                acc = 0;
                op = '+';
            }
            b'*' => {
                total += acc;
                acc = 1;
                op = '*';
            }
            b' ' => {}
            _ => panic!("Illegal char in ops: '{ch}'"),
        }
        if let Some(num) = read_num(&chars, idx) {
            if op == '+' {
                acc += num;
            } else {
                acc *= num;
            }
        }
    }

    total += acc;
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +
     */
    const INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3263827);
    }
}
