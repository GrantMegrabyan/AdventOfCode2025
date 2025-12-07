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
}
