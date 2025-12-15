use anyhow::Result;
use glam::U64Vec2 as Point;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<u64> {
    let coords: Vec<(u64, u64)> = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let area =
                (coords[i].0.abs_diff(coords[j].0) + 1) * (coords[i].1.abs_diff(coords[j].1) + 1);
            max_area = max_area.max(area);
        }
    }
    Ok(max_area)
}

#[derive(Debug)]
struct Edge {
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
}

impl From<(&Point, &Point)> for Edge {
    fn from(value: (&Point, &Point)) -> Self {
        Self {
            min_x: value.0.x.min(value.1.x),
            max_x: value.0.x.max(value.1.x),
            min_y: value.0.y.min(value.1.y),
            max_y: value.0.y.max(value.1.y),
        }
    }
}

pub fn part2(input: &str) -> Result<u64> {
    let points = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once(',').unwrap())
        .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<_>>();

    let edges: Vec<Edge> = points
        .iter()
        .zip(points.iter().skip(1).chain(points.iter().take(1)))
        .map(|pair| pair.into())
        .collect();

    let area = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            (a, b, area)
        })
        .sorted_by_key(|(_, _, area)| *area)
        .rev()
        .find(|(a, b, _)| {
            edges.iter().all(|edge| {
                let top = edge.max_y <= a.y.min(b.y);
                let bottom = edge.min_y >= a.y.max(b.y);
                let left = edge.max_x <= a.x.min(b.x);
                let right = edge.min_x >= a.x.max(b.x);
                top || bottom || left || right
            })
        })
        .unwrap()
        .2;
    Ok(area)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 24);
    }
}
