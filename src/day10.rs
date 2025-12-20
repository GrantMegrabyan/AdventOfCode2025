use std::collections::{HashSet, VecDeque};

use anyhow::Result;

struct Machine {
    target: u64,
    size: usize,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let size = value.len() - 2;
        Self {
            target: value[1..size + 1].bytes().fold(0u64, |acc, ch| {
                let d = if ch == b'.' { 0u64 } else { 1u64 };
                (acc << 1) | d
            }),
            size,
        }
    }
}

pub fn part1(input: &str) -> Result<u64> {
    let result: u64 = input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            let len = parts.len();
            let machine = Machine::from(parts[0]);
            let actions = parts[1..len - 1]
                .iter()
                .map(|act_str| {
                    act_str[1..act_str.len() - 1]
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .fold(0u64, |acc, btn| acc | 1u64 << (machine.size - btn - 1))
                })
                .collect::<Vec<_>>();
            (machine, actions)
        })
        .map(|(machine, actions)| find_min_actions(&machine, &actions))
        .sum();

    Ok(result)
}

fn find_min_actions(machine: &Machine, actions: &Vec<u64>) -> u64 {
    // (step, state, action)
    let mut queue = actions
        .iter()
        .map(|act| (0u64, 0u64, act))
        .collect::<VecDeque<_>>();
    let mut visited = HashSet::from([0]);

    while !queue.is_empty() {
        let (step, state, action) = queue.pop_front().unwrap();
        let next_state = state ^ action;
        if next_state == machine.target {
            return step + 1;
        }
        if !visited.contains(&next_state) {
            queue.extend(actions.iter().map(|act| (step + 1, next_state, act)));
        }
        visited.insert(state);
    }
    panic!("Could not reach the desired state")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 7);
    }
}
