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
fn process_beams_part_1(
    splitters: &[SplitterSpace],
    beams: &mut [BeamSpace],
    count_splits: &mut u64,
) {
    let slice_of_cells = Cell::from_mut(beams).as_slice_of_cells();

    for (beam_window, splitter) in slice_of_cells.windows(2).zip(splitters.windows(2)) {
        if beam_window[0].get() == BeamSpace::Beam && splitter[0] == SplitterSpace::Splitter {
            beam_window[0].set(BeamSpace::Empty);
            beam_window[1].set(BeamSpace::Beam);
        }

        if beam_window[1].get() == BeamSpace::Beam && splitter[1] == SplitterSpace::Splitter {
            beam_window[0].set(BeamSpace::Beam);
            *count_splits += 1;
        }
    }
}

/// Part 2 row update: propagate timeline counts through one splitter row.
///
/// Model:
/// - `count[i]` holds the number of active timelines in column i for the current row.
/// - For an empty cell, timelines continue downward unchanged.
/// - For a splitter at i, all timelines from i transfer to i-1 and i+1 in the next row.
///
/// Implementation detail:
/// - We perform neighbor transfers using a `Cell` view over the `count` slice, iterating
///   windows of size 2 alongside the splitter windows. This mirrors the Part 1 approach but
///   adds counts instead of toggling beam presence.
/// - When a splitter exists at the left index of the window and `count[left] > 0`, we move
///   those timelines to `right` and clear `left` (they do not pass straight through).
/// - When a splitter exists at the right index and `count[right] > 0`, we add those timelines
///   to `left` (the symmetric branch transfer).
fn process_beams_part_2(splitters: &[SplitterSpace], count: &mut [u64]) {
    let slice_of_cells = Cell::from_mut(count).as_slice_of_cells();

    for (count_window, splitter) in slice_of_cells.windows(2).zip(splitters.windows(2)) {
        if count_window[0].get() > 0 && splitter[0] == SplitterSpace::Splitter {
            count_window[1].set(count_window[1].get() + count_window[0].get());
            count_window[0].set(0);
        }

        if count_window[1].get() > 0 && splitter[1] == SplitterSpace::Splitter {
            count_window[0].set(count_window[0].get() + count_window[1].get());
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<StarterSpace>, Vec<Vec<SplitterSpace>>), ()> {
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

    Ok((start, splitters))
}

/// Solve part 1: parse input, simulate all rows, and return the total split count.
fn solution_part_1(input: &str) -> Result<u64, ()> {
    let (start, splitters) = parse_input(input)?;

    let mut beams = initiate_beams(start);
    let mut count_splits = 0;

    splitters
        .iter()
        .for_each(|splitter| process_beams_part_1(&splitter, &mut beams, &mut count_splits));

    Ok(count_splits)
}

/// Solve part 2: parse input, simulate the many‑worlds row updates, and sum timelines.
///
/// Steps:
/// - Convert the starter row into an initial timeline count per column (1 at each `S`).
/// - For each splitter row, apply `process_beams_part_2` to propagate counts to neighbors.
/// - At the end, sum the counts across the last row to obtain the total number of timelines.
fn solution_part_2(input: &str) -> Result<u64, ()> {
    let (start, splitters) = parse_input(input)?;

    let mut count: Vec<u64> = initiate_beams(start)
        .iter()
        .map(|&b| if b == BeamSpace::Beam { 1 } else { 0 })
        .collect();

    splitters
        .iter()
        .for_each(|splitter| process_beams_part_2(&splitter, &mut count));

    Ok(count.iter().sum())
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
        process_beams_part_1(&splitters, &mut beams, &mut total);
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
        process_beams_part_1(&splitters, &mut beams, &mut total);
        assert_eq!(
            beams,
            vec![BeamSpace::Empty, BeamSpace::Beam, BeamSpace::Empty]
        );
    }

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1(include_str!("sample_input.txt")), Ok(21));
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(solution_part_2(include_str!("sample_input.txt")), Ok(40));
    }
}
