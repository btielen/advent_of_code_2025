///! Advent of Code 2025 — Day 3: Lobby
///!
///! This module solves the "Lobby" puzzle. Each input line represents a bank of
///! single‑digit batteries (0–9). You must pick exactly `n` batteries per bank,
///! forming a number by preserving their left‑to‑right order, so that the
///! resulting number is as large as possible. The overall answer is the sum of
///! the maximum numbers for all banks.

/// Bank of batteries represented as a sequence of single digits.
///
/// The inner `Vec<u8>` stores each battery's digit value in the range 0..=9.
struct Bank(Vec<u8>);

/// Parse a `&str` of digit characters into a [`Bank`].
///
/// Any ASCII digit `0..=9` is converted to its numeric value. No other chars are
/// expected; the input should be a contiguous string of digits (e.g. `"98765"`).
impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        Bank(value.chars().map(|c| c as u8 - 48).collect())
    }
}

/// Compute the maximum possible joltage for a bank when turning on exactly `n` batteries.
///
/// The joltage is the number formed by concatenating the chosen digits in their
/// original order. This function greedily selects the left‑most possible maximum
/// for each position, ensuring the globally maximal number while respecting that
/// exactly `n` digits must be chosen.
///
/// Constraints/Behavior:
/// - `n` must be in `0..=bank.len()`. Panics if `n` is larger than the bank size.
/// - If `n == 0`, returns `0`.
fn max_jolts(bank: &Bank, n: usize) -> u64 {
    assert!(
        bank.0.len() >= n,
        "The value of n must be smaller than bank size"
    );

    if n == 0 {
        return 0;
    }

    let mut max_indexes = Vec::with_capacity(n);
    let mut last_index = 0;
    let len = bank.0.len();

    for i in 0..n {
        let new_max_index =
            first_max_value_index(&bank.0[last_index..len - n + i + 1]) + last_index;
        max_indexes.push(new_max_index);
        last_index = new_max_index + 1;
    }

    max_indexes
        .iter()
        .enumerate()
        .map(|(pow, &i)| bank.0[i] as u64 * (10u64).pow(n as u32 - pow as u32 - 1))
        .sum()
}

/// Return the index of the first maximum value in `arr`.
///
/// If multiple elements share the maximum value, the left‑most index is
/// returned. Uses a fast path for value `9`, since it is the highest possible
/// digit.
fn first_max_value_index(arr: &[u8]) -> usize {
    let mut max_index = 0;

    for (i, &x) in arr.iter().enumerate() {
        // shortcut, because 9 is always the max value
        if x == 9 {
            return i;
        }

        if x > arr[max_index] {
            max_index = i;
        }
    }

    max_index
}

/// Solve the puzzle for all banks in `input`, choosing exactly `n` batteries per bank.
///
/// The input should contain one bank per line. For each line, the function
/// computes the maximum joltage achievable by turning on exactly `n` batteries
/// (digits), and returns the sum across all lines.
fn solution(input: &str, n: usize) -> u64 {
    input
        .lines()
        .map(|line| max_jolts(&Bank::from(line), n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let bank = Bank::from("1234567890");
        assert_eq!(bank.0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_max_jolts() {
        let bank = Bank::from("987654321111111");
        assert_eq!(max_jolts(&bank, 2), 98);
    }

    #[test]
    fn test_max_jolts_max_at_end() {
        let bank = Bank::from("234234234234278"); // note: 8 at end
        assert_eq!(max_jolts(&bank, 2), 78);
    }

    #[test]
    fn test_max_jolts_with_12_batteries() {
        let bank = Bank::from("987654321111111");
        assert_eq!(max_jolts(&bank, 12), 987654321111);
    }

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution(include_str!("sample_input.txt"), 2), 357);
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(
            solution(include_str!("sample_input.txt"), 12),
            3121910778619
        );
    }
}
