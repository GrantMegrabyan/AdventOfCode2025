use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use z3::{Solver, ast::Int};

struct Machine {
    target: u64,
    jolt_target: Vec<u64>,
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
            jolt_target: vec![0; size],
            size,
        }
    }
}

impl From<(&str, &str)> for Machine {
    fn from(value: (&str, &str)) -> Self {
        let size = value.0.len() - 2;
        let jolt_size = value.1.len() - 2;
        Self {
            target: value.0[1..size + 1].bytes().fold(0u64, |acc, ch| {
                let d = if ch == b'.' { 0u64 } else { 1u64 };
                (acc << 1) | d
            }),
            jolt_target: value.1[1..jolt_size + 1]
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
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

pub fn part2(input: &str) -> Result<u64> {
    let result: u64 = input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            let len = parts.len();
            let machine = Machine::from((parts[0], parts[len - 1]));
            let actions = parts[1..len - 1]
                .iter()
                .map(|act_str| {
                    act_str[1..act_str.len() - 1]
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (machine, actions)
        })
        .map(|(machine, actions)| solve(&machine, &actions))
        .sum();
    Ok(result)
}

fn solve(machine: &Machine, actions: &Vec<Vec<usize>>) -> u64 {
    let mut buttons = vec![];
    let solver = Solver::new();
    for (idx, _) in actions.iter().enumerate() {
        let koef = Int::fresh_const(&format!("koef_{}", idx));
        solver.assert(&koef.ge(0));
        buttons.push(koef);
    }
    for (idx, jolt) in machine.jolt_target.iter().enumerate() {
        let mut sum = Int::from(0);
        for (act_idx, action) in actions.iter().enumerate() {
            if action.contains(&idx) {
                sum = sum + &buttons[act_idx];
            }
        }
        solver.assert(sum.eq(*jolt));
    }

    let mut min = u64::MAX;
    for solution in solver.solutions(buttons, true).take(500) {
        let solution: u64 = solution.iter().map(Int::as_u64).map(Option::unwrap).sum();
        min = min.min(solution);
    }
    min
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 33);
    }
}
