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
}
