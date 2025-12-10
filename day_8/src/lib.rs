///! Advent of Code Day 8 – Playground
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position(u64, u64, u64);

impl Position {
    /// Squared Euclidean distance between two 3D points.
    ///
    /// We deliberately avoid `sqrt` so we can stay in integer space and keep
    /// comparisons exact and fast.
    fn distance(&self, other: &Position) -> u64 {
        let distance_x = self.0.abs_diff(other.0);
        let distance_y = self.1.abs_diff(other.1);
        let distance_z = self.2.abs_diff(other.2);

        distance_x * distance_x + distance_y * distance_y + distance_z * distance_z
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("Missing X")?
            .parse()
            .map_err(|_| "Invalid X")?;
        let y = parts
            .next()
            .ok_or("Missing Y")?
            .parse()
            .map_err(|_| "Invalid Y")?;
        let z = parts
            .next()
            .ok_or("Missing Z")?
            .parse()
            .map_err(|_| "Invalid Z")?;
        Ok(Position(x, y, z))
    }
}

/// Strategy trait for producing the `k` closest edges between points.
///
/// Returns a vector of triples: `(distance, i, j)` where `i < j` are indices
/// into `positions`, and `distance` is the squared Euclidean distance.
trait KClosestNeighbor {
    fn closest_neighbors(&self, positions: &[Position], k: usize) -> Vec<(u64, usize, usize)>;
}

/// Simple brute-force algorithm that enumerates all O(n^2) pairs, sorts them,
/// and returns the first `k` pairs.
struct BruteForceAlgorithm;

impl KClosestNeighbor for BruteForceAlgorithm {
    fn closest_neighbors(&self, positions: &[Position], k: usize) -> Vec<(u64, usize, usize)> {
        let mut distances = Vec::new();

        for (first_index, point) in positions.iter().enumerate() {
            for (other_index, other_point) in positions.iter().enumerate().skip(first_index + 1) {
                distances.push((point.distance(other_point), first_index, other_index));
            }
        }

        distances.sort_by(|a, b| a.0.cmp(&b.0));

        distances.into_iter().take(k).collect()
    }
}

/// Parse the input where each line is `X,Y,Z`.
fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect()
}

/// Union-Find (Disjoint Set Union) to maintain circuits (connected components).
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Find the representative (root) of the set containing 'i'
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        // Recursively find the root and update the parent pointer
        // to point directly to the root (flattening the tree).
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    /// Unite the sets containing 'i' and 'j'
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        // If they are already in the same circuit, do nothing
        if root_i == root_j {
            return;
        }

        // Merge smaller set into larger set
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
    }

    /// Helper to get all circuit sizes for the final answer
    fn get_all_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            // Only collect sizes from the *roots* of the circuits
            // We use self.parent[i] here directly to check if it's a root
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }

    /// Helper to check if all points are connected
    fn all_connected(&mut self) -> bool {
        let root = self.find(0);
        self.size[root] == self.parent.len()
    }
}

/// Part 1: connect the `n` shortest edges and return the product of the sizes
/// of the three largest resulting components.
fn solution_part_1(input: &str, algorithm: impl KClosestNeighbor, n: usize) -> usize {
    let numbers = parse_input(input);
    let closest_neighbors = algorithm.closest_neighbors(&numbers, n);
    let mut uf = UnionFind::new(numbers.len());

    for (_, i, j) in closest_neighbors {
        uf.union(i, j);
    }

    let mut circuit_sizes = uf.get_all_circuit_sizes();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

/// Part 2: iterate edges in ascending order of distance and return as soon as
/// the graph becomes fully connected. The return value is the product of the
/// X-coordinates of the two points that were connected last.
///
/// The parameter `n` should be large enough to exceed the number of edges
/// needed to achieve full connectivity.
fn solution_part_2(
    input: &str,
    algorithm: impl KClosestNeighbor,
    n: usize,
) -> Result<u64, &'static str> {
    let numbers = parse_input(input);
    let closest_neighbors = algorithm.closest_neighbors(&numbers, n);
    let mut uf = UnionFind::new(numbers.len());

    for (_, i, j) in closest_neighbors {
        uf.union(i, j);
        if uf.all_connected() {
            return Ok(numbers[i].0 * numbers[j].0);
        }
    }

    Err("Could not connect all points")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(
            solution_part_1(include_str!("sample_input.txt"), BruteForceAlgorithm, 10),
            40
        );
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(
            // note that we use a magic number here to reuse part1, which
            // could be considered as a dirty hack
            solution_part_2(
                include_str!("sample_input.txt"),
                BruteForceAlgorithm,
                10_000
            ),
            Ok(25272)
        );
    }
}
