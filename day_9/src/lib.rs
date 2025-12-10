///! Advent of Code Day 9 – Movie Theater
use std::str::FromStr;

/// A point on the theater floor grid.
struct Point(u64, u64);

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("Missing comma")?;

        let x = x.parse().map_err(|_| "Invalid X")?;
        let y = y.parse().map_err(|_| "Invalid Y")?;

        Ok(Point(x, y))
    }
}

/// Parses the raw puzzle input into a list of `Point`s.
fn parse_input(input: &str) -> Result<Vec<Point>, &'static str> {
    input.lines().map(|line| line.parse()).collect()
}

/// Computes the axis-aligned rectangle area defined by two opposite corners.
///
/// Example
/// - Opposite corners `(2,5)` and `(11,1)` yield width `(11-2)+1 = 10`, height `(5-1)+1 = 5`,
///   so area is `10 * 5 = 50`.
fn area(point: &Point, other: &Point) -> u64 {
    (point.0.abs_diff(other.0) + 1) * (point.1.abs_diff(other.1) + 1)
}

/// Part 1 solution: find the largest rectangle area using any two red tiles
/// as opposite corners.
///
/// Complexity
/// - Time: O(n²) over the number of points
/// - Space: O(1)
fn solution_part_1(input: &str) -> u64 {
    let points = parse_input(input).expect("Failed to parse input");

    let mut max = 0;

    // todo: this is the same as day 8, we could spend some time to optimize it
    for (index, point) in points.iter().enumerate() {
        for other_point in points.iter().skip(index + 1) {
            let area = area(point, other_point);
            if area > max {
                max = area;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(area(&Point(2, 5), &Point(11, 1)), 50);
    }

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1(include_str!("sample_input.txt")), 50);
    }
}
