///! Advent of Code 2025 — Day 6: Trash Compactor

/// Errors that can occur while parsing or evaluating the Day 6 worksheet.
#[derive(Debug, PartialEq)]
enum Day6Error {
    /// The provided input had no lines.
    EmptyInput,
    /// An operator other than `+` or `*` was encountered.
    UnknownOperator,
    /// A block did not contain any operator in its bottom row.
    OperatorNotFound,
    /// A numeric token could not be parsed into an integer.
    ParseIntError,
}

/// The operator that applies to a block of numbers.
enum Operator {
    /// Sum all numbers in the block.
    Addition,
    /// Multiply all numbers in the block.
    Multiplication,
}

impl TryFrom<&str> for Operator {
    type Error = Day6Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Addition),
            "*" => Ok(Operator::Multiplication),
            _ => Err(Day6Error::UnknownOperator),
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = Day6Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operator::Addition),
            '*' => Ok(Operator::Multiplication),
            _ => Err(Day6Error::UnknownOperator),
        }
    }
}

/// Parse a whitespace-separated list of decimal numbers from a single line.
fn parse_numbers(input: &str) -> Result<Vec<u64>, Day6Error> {
    input
        .split_whitespace()
        .map(|str| str.parse::<u64>())
        .collect::<Result<_, _>>()
        .map_err(|_| Day6Error::ParseIntError)
}

/// Read the first item from a reversed line iterator (which is the original
/// bottom line) and parse its tokens as `Operator`s.
fn get_operators<'a>(mut iter: impl Iterator<Item = &'a str>) -> Result<Vec<Operator>, Day6Error> {
    iter.next()
        .ok_or(Day6Error::EmptyInput)?
        .split_whitespace()
        .map(Operator::try_from)
        .collect::<Result<_, _>>()
}

/// Part 1: Evaluate problems defined in vertical stacks, separated by
/// columns of spaces, reading numbers top-to-bottom and operators from the
/// last line (original bottom). For each column index:
/// - If operator is `+`, add all numbers in that column to the running sum.
/// - If operator is `*`, multiply numbers within that column together, then
///   add the resulting product to the final answer at the end.
fn solution_part_1(input: &str) -> Result<u64, Day6Error> {
    let mut reversed_lines = input.lines().rev();
    let operators = get_operators(&mut reversed_lines)?;

    let mut sum = 0;
    let mut multiplications: Vec<u64> = vec![1; operators.len()];

    for line in reversed_lines {
        let numbers: Vec<u64> = parse_numbers(line)?;

        for (index, &number) in numbers.iter().enumerate() {
            match operators[index] {
                Operator::Addition => sum += number,
                Operator::Multiplication => {
                    multiplications[index] = multiplications[index] * number
                }
            }
        }
    }

    // Sum the products only for columns that actually performed multiplication
    let total_multiplications = multiplications.into_iter().filter(|&n| n != 1).sum::<u64>();

    Ok(sum + total_multiplications)
}

/// Part 2: Cephalopod math reads right-to-left, with each number given in its
/// own column. We:
/// - Convert the input to a ragged 2D grid of chars.
/// - Scan columns left-to-right to find contiguous non-space "blocks" that
///   represent one problem each.
/// - For a block, detect its operator from the bottom row, and parse every
///   column above into a number by reading digit chars from top to bottom.
/// - Apply the operator over all parsed numbers and sum across all blocks.
fn solution_part_2(input: &str) -> Result<u64, Day6Error> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    if grid.is_empty() {
        return Err(Day6Error::EmptyInput);
    }

    let height = grid.len();
    let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut total = 0;
    let mut current_block = Vec::new();

    // Iterate through columns
    for col in 0..=width {
        let is_empty = (0..height).all(|y| {
            grid.get(y)
                .and_then(|row| row.get(col))
                .map_or(true, |&c| c == ' ')
        });

        if !is_empty && col < width {
            current_block.push(col);
        } else if !current_block.is_empty() {
            total += solve_block(&grid, &current_block)?;
            current_block.clear();
        }
    }

    Ok(total)
}

/// Given a set of contiguous column indices (`cols`) that form one problem,
/// determine its operator from the bottom row and compute the result by
/// applying it to all per-column numbers above.
fn solve_block(grid: &[Vec<char>], cols: &[usize]) -> Result<u64, Day6Error> {
    let height = grid.len();
    let operator_row = height - 1;

    let operator = cols
        .iter()
        .find_map(|&x| {
            grid.get(operator_row)
                .and_then(|row| row.get(x))
                .and_then(|&c| Operator::try_from(c).ok())
        })
        .ok_or(Day6Error::OperatorNotFound)?;

    let numbers = cols
        .iter()
        .map(|&col| parse_number_in_column(&grid[..height - 1], col))
        // Optional: filter out completely empty number columns if necessary
        .filter(|&n| n > 0);

    match operator {
        Operator::Addition => Ok(numbers.sum()),
        Operator::Multiplication => Ok(numbers.product()),
    }
}

/// Parse one number from a single column by reading digits top-to-bottom and
/// treating them as most-significant to least-significant.
fn parse_number_in_column(grid: &[Vec<char>], col: usize) -> u64 {
    (0..grid.len())
        .filter_map(|y| {
            grid.get(y)
                .and_then(|row| row.get(col))
                .and_then(|c| c.to_digit(10))
        })
        .fold(0, |acc, digit| acc * 10 + digit as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(
            solution_part_1(include_str!("sample_input.txt")),
            Ok(4277556)
        );
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(
            solution_part_2(include_str!("sample_input.txt")),
            Ok(3263827)
        );
    }
}
