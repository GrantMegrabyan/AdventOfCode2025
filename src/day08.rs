use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        f64::sqrt(
            (self.x - other.x).pow(2) as f64
                + (self.y - other.y).pow(2) as f64
                + (self.z - other.z).pow(2) as f64,
        )
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.split(',')
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .ok()
            .map(|[a, b, c]: [&str; 3]| Self {
                x: a.parse().unwrap(),
                y: b.parse().unwrap(),
                z: c.parse().unwrap(),
            })
            .ok_or("invalid format".to_string())
    }
}

#[derive(Debug)]
struct Pair {
    p1: Point,
    p2: Point,
    d: f64,
}

#[derive(Debug)]
struct Circuits {
    circuits: Vec<HashSet<Point>>,
    points: HashMap<Point, usize>,
}

impl Circuits {
    pub fn new() -> Self {
        Self {
            circuits: Vec::new(),
            points: HashMap::new(),
        }
    }

    pub fn add(&mut self, p1: &Point, p2: &Point) {
        if self.points.contains_key(p1) && self.points.contains_key(p2) {
            let circ_id1 = self.points[p1];
            let circ_id2 = self.points[p2];
            let points_to_move = self.circuits[circ_id2].iter().cloned().collect::<Vec<_>>();
            for point in points_to_move {
                self.circuits[circ_id1].insert(point);
                self.points.entry(point).and_modify(|id| *id = circ_id1);
            }
            self.circuits[circ_id2].clear();
        } else if let Some(circ_id) = self.points.get(p1) {
            self.circuits[*circ_id].insert(*p2);
            self.points.insert(*p2, *circ_id);
        } else if let Some(circ_id) = self.points.get(p2) {
            self.circuits[*circ_id].insert(*p1);
            self.points.insert(*p1, *circ_id);
        } else {
            let new_id = self.circuits.len();
            self.circuits.push(HashSet::new());
            self.circuits[new_id].insert(*p1);
            self.circuits[new_id].insert(*p2);
            self.points.insert(*p1, new_id);
            self.points.insert(*p2, new_id);
        }
    }

    pub fn contains(&self, p1: &Point, p2: &Point) -> bool {
        if let Some(circ_id) = self.points.get(p1) {
            self.circuits[*circ_id].contains(p2)
        } else if let Some(circ_id) = self.points.get(p2) {
            self.circuits[*circ_id].contains(p1)
        } else {
            false
        }
    }
}

fn get_closest(connections: &HashSet<(Point, Point)>, points: &[Point]) -> Pair {
    if points.len() == 1 {
        return Pair {
            p1: points[0],
            p2: points[0],
            d: f64::MAX,
        };
    }
    if points.len() == 2 {
        if connections.contains(&(points[0], points[1]))
            || connections.contains(&(points[1], points[0]))
        {
            return Pair {
                p1: points[0],
                p2: points[0],
                d: f64::MAX,
            };
        }
        return Pair {
            p1: points[0],
            p2: points[1],
            d: points[0].dist(&points[1]),
        };
    }
    let mid = points.len() / 2;
    let pair1 = get_closest(connections, &points[..mid]);
    let pair2 = get_closest(connections, &points[mid..]);

    let mut closest = if pair1.d < pair2.d { pair1 } else { pair2 };

    let strip_x = (points[mid - 1].x + points[mid].x) / 2;
    let mut strip = points
        .iter()
        .filter(|p| p.x.abs_diff(strip_x) as f64 <= closest.d)
        .collect::<Vec<_>>();
    strip.sort_by_key(|p| p.y);

    for (idx, &p) in strip.iter().enumerate() {
        for &next in &strip[idx + 1..(idx + 16).min(strip.len())] {
            if connections.contains(&(*p, *next)) || connections.contains(&(*next, *p)) {
                continue;
            }
            let dist = p.dist(next);
            if dist < closest.d {
                closest = Pair {
                    p1: *p,
                    p2: *next,
                    d: dist,
                }
            }
        }
    }
    closest
}

pub fn part1(input: &str, iterations: usize) -> Result<i32> {
    let mut points: Vec<Point> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    points.sort_by_key(|p| p.x);

    let mut circuits = Circuits::new();
    let mut connections = HashSet::new();

    for _ in 0..iterations {
        let closest = get_closest(&connections, &points);
        if !circuits.contains(&closest.p1, &closest.p2) {
            circuits.add(&closest.p1, &closest.p2);
        }
        connections.insert((closest.p1, closest.p2));
    }

    let mut sizes = circuits
        .circuits
        .iter()
        .map(|c| c.len() as i32)
        .collect::<Vec<i32>>();
    sizes.sort();
    sizes.reverse();

    Ok(sizes.iter().take(3).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let result = part1(INPUT, 10);
        assert_eq!(result.unwrap(), 40);
    }
}
