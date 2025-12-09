//! Advent of Code Day 7 — Laboratories

use std::cell::Cell;

/// Per-position beam state for a single row.
#[derive(Debug, PartialEq, Clone, Copy)]
enum BeamSpace {
    Empty,
    Beam,
}

/// Per-position splitter state for a single row.
#[derive(Debug, PartialEq)]
enum SplitterSpace {
    Empty,
    Splitter,
}

/// Per-position starter state for the top row.
#[derive(Debug, PartialEq)]
enum StarterSpace {
    Empty,
    Starter,
}

impl TryFrom<char> for BeamSpace {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(BeamSpace::Beam),
            '.' => Ok(BeamSpace::Empty),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for SplitterSpace {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(SplitterSpace::Splitter),
            '.' => Ok(SplitterSpace::Empty),
            _ => Err(()),
        }
    }
}

impl From<StarterSpace> for BeamSpace {
    /// Convert a starter position into an initial beam state.
    fn from(value: StarterSpace) -> Self {
        match value {
            StarterSpace::Starter => BeamSpace::Beam,
            StarterSpace::Empty => BeamSpace::Empty,
        }
    }
}

impl TryFrom<char> for StarterSpace {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(StarterSpace::Starter),
            '.' => Ok(StarterSpace::Empty),
            _ => Err(()),
        }
    }
}

/// Initialize the beam row from the starter row.
fn initiate_beams(input: Vec<StarterSpace>) -> Vec<BeamSpace> {
    input.into_iter().map(BeamSpace::from).collect()
}

/// Apply one splitter row to the current beam row in place and update the split count.
///
/// For each adjacent pair (i, i+1), we examine whether a beam at i hits a splitter at i.
/// If so, that incoming beam stops at i and produces a rightward beam at i+1.
/// Symmetrically, if there is a beam at i+1 hitting a splitter at i+1, we produce
/// a leftward beam at i and increment the split counter by one.
fn process_beams(splitters: &[SplitterSpace], beams: &mut [BeamSpace], count_splits: &mut u64) {
    let slice_of_cells = Cell::from_mut(beams).as_slice_of_cells();

    for (beams, splitter) in slice_of_cells.windows(2).zip(splitters.windows(2)) {
        if beams[0].get() == BeamSpace::Beam && splitter[0] == SplitterSpace::Splitter {
            beams[0].set(BeamSpace::Empty);
            beams[1].set(BeamSpace::Beam);
        }

        if beams[1].get() == BeamSpace::Beam && splitter[1] == SplitterSpace::Splitter {
            beams[0].set(BeamSpace::Beam);
            *count_splits += 1;
        }
    }
}

/// Solve part 1: parse input, simulate all rows, and return the total split count.
fn solution_part_1(input: &str) -> Result<u64, ()> {
    let mut iter = input.lines().step_by(2);
    let start: Vec<StarterSpace> = iter
        .next()
        .ok_or(())?
        .chars()
        .map(StarterSpace::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let splitters: Vec<Vec<SplitterSpace>> = iter
        .map(|line| line.chars().map(SplitterSpace::try_from).collect())
        .collect::<Result<Vec<_>, _>>()?;

    let mut beams = initiate_beams(start);
    let mut count_splits = 0;

    splitters
        .iter()
        .for_each(|splitter| process_beams(&splitter, &mut beams, &mut count_splits));

    Ok(count_splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_beams_split() {
        let mut beams = vec![BeamSpace::Empty, BeamSpace::Beam, BeamSpace::Empty];
        let splitters = vec![
            SplitterSpace::Empty,
            SplitterSpace::Splitter,
            SplitterSpace::Empty,
        ];
        let mut total = 0;
        process_beams(&splitters, &mut beams, &mut total);
        assert_eq!(
            beams,
            vec![BeamSpace::Beam, BeamSpace::Empty, BeamSpace::Beam]
        );
    }

    #[test]
    fn test_process_beams_straight_beam() {
        let mut beams = vec![BeamSpace::Empty, BeamSpace::Beam, BeamSpace::Empty];
        let splitters = vec![
            SplitterSpace::Splitter,
            SplitterSpace::Empty,
            SplitterSpace::Empty,
        ];

        let mut total = 0;
        process_beams(&splitters, &mut beams, &mut total);
        assert_eq!(
            beams,
            vec![BeamSpace::Empty, BeamSpace::Beam, BeamSpace::Empty]
        );
    }

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1(include_str!("sample_input.txt")), Ok(21));
    }
}
