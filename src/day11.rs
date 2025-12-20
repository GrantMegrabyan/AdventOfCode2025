use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    let graph = input
        .trim()
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .fold(HashMap::new(), |mut acc, (from, to)| {
            let to = to.split(" ").collect::<Vec<_>>();
            acc.insert(from, to);
            acc
        });

    let mut memo = HashMap::new();
    let result = dfs(&graph, "you", &mut memo);
    Ok(result)
}

fn dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if from == "out" {
        return 1;
    }
    if memo.contains_key(from) {
        return memo[from];
    }

    let paths = graph[from]
        .iter()
        .map(|node| dfs(graph, node, memo))
        .sum::<usize>();

    memo.insert(from, paths);
    paths
}

pub fn part2(input: &str) -> Result<usize> {
    let graph = input
        .trim()
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .fold(HashMap::new(), |mut acc, (from, to)| {
            let to = to.split(" ").collect::<Vec<_>>();
            acc.insert(from, to);
            acc
        });

    let mut memo = HashMap::new();
    let result = dfs2(&graph, "svr", &mut memo, 0);
    Ok(result)
}

const FFT: u8 = 0b1;
const DAC: u8 = 0b10;
const BOTH: u8 = 0b11;

fn dfs2<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    memo: &mut HashMap<(&'a str, u8), usize>,
    visits: u8,
) -> usize {
    if from == "out" {
        return if visits == BOTH { 1 } else { 0 };
    }
    let memo_key = (from, visits);
    if memo.contains_key(&memo_key) {
        return memo[&memo_key];
    }

    let mut visits = visits;
    if from == "fft" {
        visits |= FFT
    }
    if from == "dac" {
        visits |= DAC
    }

    let paths = graph[from]
        .iter()
        .map(|node| dfs2(graph, node, memo, visits))
        .sum::<usize>();

    memo.insert((from, visits), paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 5);
    }

    const INPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2).unwrap(), 2);
    }
}
