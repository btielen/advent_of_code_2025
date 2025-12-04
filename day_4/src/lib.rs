///! Advent of Code 2025 - Day 4: Printing Department
use std::collections::HashMap;

/// Maximum number of adjacent rolls a roll can have before it becomes
/// inaccessible to a forklift.
const LIMIT_NEIGHBOURS: usize = 4;

#[derive(Copy, Clone, PartialEq, Debug)]
/// A space type present in the grid.
///
/// Currently, the grid only stores occupied cells (paper rolls). Empty cells
/// are not stored in the sparse representation and thus have no explicit
/// variant here.
enum Space {
    PaperRoll,
}

/// Row/Column coordinate used to address positions in the grid.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate(i16, i16);

/// Sparse grid mapping coordinates to occupied spaces (paper rolls).
///
/// Only cells that contain a paper roll are stored to keep memory usage low
/// for large inputs dominated by empty cells.
struct Grid(HashMap<Coordinate, Space>);

impl Grid {
    /// Create a new grid with no spaces
    fn new() -> Self {
        Grid(HashMap::new())
    }

    /// Get all spaces adjacent to a coordinate
    fn neighbour_spaces(&self, coordinate: &Coordinate) -> [Option<&Space>; 8] {
        coordinate
            .neighbour_coordinates()
            .map(|coord| self.get_space(&coord))
    }

    /// Returns the space located at `coordinate`, if any
    fn get_space(&self, coordinate: &Coordinate) -> Option<&Space> {
        self.0.get(coordinate)
    }

    /// Returns an iterator over all coordinates that contain a paper roll
    fn coordinates(&self) -> impl Iterator<Item = &Coordinate> {
        self.0.keys()
    }
}

impl Coordinate {
    /// Create coordinate
    fn new(row: i16, col: i16) -> Self {
        Coordinate(row, col)
    }

    /// Get all neighboring coordinates
    fn neighbour_coordinates(&self) -> [Coordinate; 8] {
        let row = self.0;
        let col = self.1;

        [
            Coordinate::new(row - 1, col - 1),
            Coordinate::new(row - 1, col),
            Coordinate::new(row - 1, col + 1),
            Coordinate::new(row, col - 1),
            Coordinate::new(row, col + 1),
            Coordinate::new(row + 1, col - 1),
            Coordinate::new(row + 1, col),
            Coordinate::new(row + 1, col + 1),
        ]
    }
}

/// Count the number of paper rolls in a slice of spaces
fn count_paper_rolls(spaces: &[Option<&Space>]) -> usize {
    spaces.iter().filter(|&space| space.is_some()).count()
}

/// Solves part 1: count paper rolls with fewer than `MAX_NEIGHBOURS`
/// adjacent rolls in the initial grid.
fn solution_part_1(input: &str) -> usize {
    let grid = Grid::try_from(input).expect("Failed to parse input");
    let counter = NeighbourCount::from(&grid);

    counter.accessible_coordinates().iter().count()
}

/// Tracks, for each paper roll coordinate, how many neighbouring rolls it has.
///
/// This supports efficiently finding and updating accessible rolls as removals
/// happen during Part 2.
struct NeighbourCount {
    map: HashMap<Coordinate, usize>,
}

impl NeighbourCount {
    /// Constructs an empty neighbour counter map
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Decrease the neighbour count for all neighbours of `coordinate`
    ///
    /// This should be called when a paper roll at `coordinate` is removed,
    /// since all eight adjacent positions lose one neighbouring roll.
    fn decrease_neighbours_count(&mut self, coordinate: &Coordinate) {
        // collect neighbour coordinates of the removed coordinate
        let neighbours = coordinate.neighbour_coordinates();

        for neighbour in neighbours {
            self.map
                .entry(neighbour)
                .and_modify(|c| *c = c.saturating_sub(1));
        }
    }

    /// Collects all coordinates that are currently accessible to forklifts
    /// (i.e., that have fewer than `MAX_NEIGHBOURS` adjacent paper rolls).
    fn accessible_coordinates(&self) -> Vec<Coordinate> {
        self.map
            .iter()
            .filter(|(_, count)| *count < &LIMIT_NEIGHBOURS)
            .map(|(coord, _)| *coord)
            .collect()
    }
}

impl From<&Grid> for NeighbourCount {
    /// Builds a `NeighbourCount` map by counting neighbours for each paper roll
    fn from(grid: &Grid) -> Self {
        grid.coordinates()
            .fold(NeighbourCount::new(), |mut acc, coord| {
                acc.map
                    .insert(*coord, count_paper_rolls(&grid.neighbour_spaces(&coord)));
                acc
            })
    }
}

/// Solves part 2: repeatedly remove all currently accessible paper rolls
/// (having fewer than `MAX_NEIGHBOURS` neighbours), updating neighbour counts
/// after each wave, and return the total number of removed rolls.
fn solution_part_2(input: &str) -> usize {
    let grid = Grid::try_from(input).expect("Failed to parse input");
    let mut counter = NeighbourCount::from(&grid);
    let mut total_removed = 0;

    loop {
        let candidates = counter.accessible_coordinates();
        if candidates.is_empty() {
            break;
        }

        total_removed += candidates.len();

        for coordinate in candidates {
            counter.decrease_neighbours_count(&coordinate);
            counter.map.remove(&coordinate);
        }
    }

    total_removed
}

#[derive(Debug)]
/// Errors that can occur while parsing input into the grid representation.
enum ParsingError {
    UnknownSpaceChar,
    CoordinateOutOfBounds,
}

impl TryFrom<char> for Space {
    type Error = ParsingError;

    /// Parse a single character into a `Space` variant.
    ///
    /// Currently, only `'@'` is recognized as a paper roll; `'.'` and other
    /// characters should be filtered out by the grid parser and will cause an
    /// error if passed here.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Space::PaperRoll),
            _ => Err(ParsingError::UnknownSpaceChar),
        }
    }
}

impl TryFrom<&str> for Grid {
    type Error = ParsingError;

    /// Parse the puzzle input into a sparse `Grid`.
    ///
    /// Each line is a row; `'@'` denotes a paper roll which is stored,
    /// `'.'` is empty and skipped. Coordinates are 0-based `(row, col)`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = Grid::new();

        for (row, line) in value.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }

                let row = i16::try_from(row).map_err(|_| ParsingError::CoordinateOutOfBounds)?;
                let col = i16::try_from(col).map_err(|_| ParsingError::CoordinateOutOfBounds)?;

                grid.0
                    .insert(Coordinate::new(row, col), Space::try_from(char)?);
            }
        }

        Ok(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1(include_str!("sample_input.txt")), 13);
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(solution_part_2(include_str!("sample_input.txt")), 43);
    }
}
