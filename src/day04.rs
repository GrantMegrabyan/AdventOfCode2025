use anyhow::Result;

pub fn part1(input: &str) -> Result<u64> {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn get_neighbours(arr: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<&char> {
        Vec::<(i32, i32)>::from([
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ])
        .into_iter()
        .map(|(dx, dy)| (i as i32 + dx, j as i32 + dy))
        .filter(|(x, y)| *x >= 0 && *x < arr.len() as i32 && *y >= 0 && *y < arr[0].len() as i32)
        .map(|(x, y)| &arr[x as usize][y as usize])
        .collect::<Vec<&char>>()
    }

    let mut cnt = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let char = arr[i][j];
            if char == '@' {
                let neigh = get_neighbours(&arr, i, j);
                let papers = neigh.iter().filter(|&&c| *c == '@').count();
                if papers < 4 {
                    cnt += 1;
                }
            }
        }
    }
    Ok(cnt)
}

pub fn part2(input: &str) -> Result<u64> {
    let mut arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn get_neighbours(arr: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<&char> {
        Vec::<(i32, i32)>::from([
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ])
        .into_iter()
        .map(|(dx, dy)| (i as i32 + dx, j as i32 + dy))
        .filter(|(x, y)| *x >= 0 && *x < arr.len() as i32 && *y >= 0 && *y < arr[0].len() as i32)
        .map(|(x, y)| &arr[x as usize][y as usize])
        .collect::<Vec<&char>>()
    }

    let mut cnt = 0;
    loop {
        let mut remove = vec![];
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                let char = arr[i][j];
                if char == '@' {
                    let neigh = get_neighbours(&arr, i, j);
                    let papers = neigh.iter().filter(|&&c| *c == '@').count();
                    if papers < 4 {
                        remove.push((i, j));
                    }
                }
            }
        }

        if !remove.is_empty() {
            remove.iter().for_each(|(i, j)| arr[*i][*j] = '.');
            cnt += remove.iter().count() as u64;
        } else {
            break;
        }
    }
    Ok(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 43);
    }
}
