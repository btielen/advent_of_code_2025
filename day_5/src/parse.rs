// Parsing helpers for Day 5: Cafeteria
//
// Expected input format:
// <min>-<max>      (multiple lines; inclusive ranges)
// ...
//
// <id>             (blank line separates sections; list of available IDs)
// ...
//
// This module parses the text into `MultipleRanges` and `Vec<u64>` for use by the
// solution functions in `lib.rs`.

use crate::{MultipleRanges, Range};

/// Possible parsing errors for the Day 5 input format.
#[derive(Debug)]
pub enum ParseError {
    InvalidInputFormat,
    InvalidRange,
    InvalidNumber,
}

/// Parse the entire puzzle input into (ranges, available_ids).
pub fn parse_input(input: &str) -> Result<(MultipleRanges, Vec<u64>), ParseError> {
    let (ranges, ids) = input
        .split_once("\n\n")
        .ok_or(ParseError::InvalidInputFormat)?;

    let ranges = MultipleRanges::try_from(ranges)?;

    let ids = ids
        .lines()
        .map(|id| id.parse().map_err(|_| ParseError::InvalidNumber))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((ranges, ids))
}

impl TryFrom<&str> for Range {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (min, max) = value.split_once('-').ok_or(ParseError::InvalidRange)?;

        let min: u64 = min.parse().map_err(|_| ParseError::InvalidNumber)?;
        let max: u64 = max.parse().map_err(|_| ParseError::InvalidNumber)?;

        Ok(Self::new(min, max))
    }
}

impl TryFrom<&str> for MultipleRanges {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ranges = value
            .lines()
            .map(Range::try_from)
            .collect::<Result<_, _>>()?;

        Ok(MultipleRanges::new(ranges))
    }
}
