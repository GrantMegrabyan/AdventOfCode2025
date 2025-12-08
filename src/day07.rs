use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    // a[10].any(|d| d < 6)
    let coords = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| (row, col, char))
                .collect::<Vec<(usize, usize, char)>>()
        })
        .filter(|(_, _, ch)| *ch != '.')
        .fold(HashMap::new(), |mut map, (row, col, ch)| {
            if ch == '^' {
                // Check if it is directly below S
                for i in (0..row).rev() {
                    if map.contains_key(&(i, col - 1)) && map[&(i, col - 1)] == '^' {
                        map.insert((row, col), '^');
                        break;
                    }
                    if map.contains_key(&(i, col + 1)) && map[&(i, col + 1)] == '^' {
                        map.insert((row, col), '^');
                        break;
                    }
                    if map.contains_key(&(i, col)) {
                        if map[&(i, col)] == 'S' {
                            map.insert((row, col), '^');
                            break;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                map.insert((row, col), 'S');
            }
            map
        });

    Ok(coords.len() as u64 - 1)
}

pub fn part2(input: &str) -> Result<u64> {
    fn count_paths(map: &HashMap<(usize, usize), (char, u64)>, row: usize, col: usize) -> u64 {
        let mut paths = 0u64;
        for i in (0..row).rev() {
            if col > 0 && map.contains_key(&(i, col - 1)) && matches!(map[&(i, col - 1)], ('^', _))
            {
                paths += map[&(i, col - 1)].1;
            }
            if map.contains_key(&(i, col + 1)) && matches!(map[&(i, col + 1)], ('^', _)) {
                paths += map[&(i, col + 1)].1;
            }
            if map.contains_key(&(i, col)) {
                if matches!(map[&(i, col)], ('S', _)) {
                    paths += map[&(i, col)].1;
                }
                break;
            }
        }
        paths
    }

    let mut rows = 0;
    let mut cols = 0;
    let coords = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let chars = line
                .chars()
                .enumerate()
                .map(|(col, char)| (row, col, char))
                .collect::<Vec<(usize, usize, char)>>();
            cols = chars.len();
            rows += 1;
            chars
        })
        .filter(|(_, _, ch)| *ch != '.')
        .fold(HashMap::new(), |mut map, (row, col, ch)| {
            if ch == '^' {
                let paths = count_paths(&map, row, col);
                if paths != 0 {
                    map.insert((row, col), ('^', paths));
                }
            } else {
                map.insert((row, col), ('S', 1));
            }
            map
        });

    let timelines: u64 = (0..cols)
        .map(|col| count_paths(&coords, rows + 1, col))
        .sum();

    Ok(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 40);
    }
}
