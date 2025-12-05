//! Advent of Code - Day 5: Cafeteria

use crate::parse::parse_input;
use std::cmp::Ordering;

mod parse;

/// A closed interval [min, max] representing fresh ingredient IDs.
#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    min: u64,
    max: u64,
}
impl Range {
    /// Construct a new closed interval [min, max].
    fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    /// Whether `value` lies inside [min, max].
    fn contains(&self, value: u64) -> bool {
        value >= self.min && value <= self.max
    }

    /// Two closed intervals overlap if they share at least one point.
    fn is_overlapping(&self, other: &Self) -> bool {
        self.max >= other.min && self.min <= other.max
    }

    /// Merge two overlapping ranges into their union; return `None` if disjoint.
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.is_overlapping(other) {
            Some(Self::new(self.min.min(other.min), self.max.max(other.max)))
        } else {
            None
        }
    }

    /// Size of the closed interval: max - min + 1
    fn size(&self) -> u64 {
        self.max - self.min + 1
    }
}

/// A set of ranges. After `merge_overlapping`, the inner vector is pairwise-disjoint
/// and sorted by (min, max).
struct MultipleRanges(Vec<Range>);

impl MultipleRanges {
    /// Construct from a raw vector of (possibly unsorted/overlapping) ranges.
    fn new(ranges: Vec<Range>) -> Self {
        Self(ranges)
    }

    /// Check if any range contains `value`.
    fn contains(&self, value: u64) -> bool {
        self.0.iter().any(|range| range.contains(value))
    }

    /// In-place merge of overlapping ranges.
    ///
    /// Algorithm:
    /// - Sort ranges by (min, max).
    /// - Sweep once, merging the current range with the next if they overlap,
    ///   otherwise flush the current range to the result and continue.
    fn merge_overlapping(&mut self) {
        if self.0.is_empty() {
            return;
        }
        self.0.sort();

        let mut merged: Vec<Range> = Vec::new();

        let mut current = self.0[0].clone();

        for next_range in self.0.iter().skip(1) {
            if let Some(m) = current.merge(next_range) {
                // If they overlap, update `current` to be the merged version
                current = m;
            } else {
                // If they don't, push 'current' to results and start a new 'current'
                merged.push(current);
                current = next_range.clone();
            }
        }
        merged.push(current);

        self.0 = merged;
    }
}

impl PartialOrd<Self> for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.min, self.max).cmp(&(other.min, other.max))
    }
}

fn solution_part_1(input: &str) -> usize {
    let (ranges, ids) = parse_input(input).expect("Failed to parse input");

    ids.iter().filter(|&id| ranges.contains(*id)).count()
}

fn solution_part_2(input: &str) -> u64 {
    let (mut ranges, _) = parse_input(input).expect("Failed to parse input");

    ranges.merge_overlapping();

    ranges.0.iter().map(|range| range.size()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1(include_str!("sample_input.txt")), 3);
    }

    #[test]
    fn test_merge_overlapping_with_overlapping_ranges() {
        let mut ranges = MultipleRanges::new(vec![Range::new(10, 14), Range::new(12, 18)]);
        ranges.merge_overlapping();
        assert_eq!(ranges.0, vec![Range::new(10, 18)])
    }

    #[test]
    fn test_merge_overlapping_with_disjoint_ranges() {
        let mut ranges = MultipleRanges::new(vec![Range::new(2, 5), Range::new(12, 18)]);
        ranges.merge_overlapping();

        assert_eq!(ranges.0, vec![Range::new(2, 5), Range::new(12, 18)],)
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(solution_part_2(include_str!("sample_input.txt")), 14);
    }
}
