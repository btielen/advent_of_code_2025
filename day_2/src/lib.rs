///! Advent of Code Day 2 - Gift Shop

/// Returns `true` if `id` is valid for Part 1 rules (not exactly two equal halves).
///
/// Logic:
/// - If the length is odd, it cannot be two equal halves → valid.
/// - Otherwise compare the first half and the second half; if equal → invalid, else valid.
fn is_valid_part_1(id: &str) -> bool {
    let len = id.len();

    // if the length is odd, the id is always valid
    if len & 1 != 0 {
        return true;
    }

    // otherwise, the id is valid if the first half of the digits are different from the second half
    id[0..len / 2] != id[len / 2..]
}

/// Returns `true` if `id` is valid for Part 2 rules (not k repeats for any k ≥ 2).
///
/// Logic:
/// - Enumerate all proper divisors of the length (possible part sizes).
/// - If any partitioning into equal-sized chunks has all chunks equal → invalid.
fn is_valid_part_2(id: &str) -> bool {
    let dividers = dividers(id.len());

    for divider in dividers {
        if parts_are_equal(id, divider) {
            return false;
        }
    }

    true
}

/// Check if all chunks of length `part_len` in `value` equal the first chunk.
/// Assumes `part_len` divides `value.len()`.
fn parts_are_equal(value: &str, part_len: usize) -> bool {
    let n = value.len() / part_len;
    let first = &value[0..part_len];

    (0..n)
        .map(|i| &value[i * part_len..(i + 1) * part_len])
        .all(|x| x == first)
}

/// Get all proper divisors of `n` (values in 1..n that divide `n`).
fn dividers(n: usize) -> Vec<usize> {
    (1..n).filter(|&x| n % x == 0).collect()
}

/// Parse a range from a string of the form `start-end`.
fn min_max(input: &str) -> (u64, u64) {
    let id_range: Vec<&str> = input.split('-').collect();
    let min: u64 = id_range[0].parse().expect("Could not parse start of range");
    let max: u64 = id_range[1].parse().expect("Could not parse end of range");

    (min, max)
}

/// Brute-force solution for Part 1: sum all invalid IDs across the input ranges.
fn bruteforce_solution_part_1(input: &str) -> u64 {
    let mut total = 0;
    let ranges: Vec<&str> = input.split(',').collect();

    for range in ranges {
        let (min, max) = min_max(range);

        for id in min..=max {
            if !is_valid_part_1(&id.to_string()) {
                total += id;
            }
        }
    }

    total
}

/// Brute-force solution for Part 2: sum all invalid IDs across the input ranges.
fn bruteforce_solution_part_2(input: &str) -> u64 {
    let mut total = 0;
    let ranges: Vec<&str> = input.split(',').collect();

    for range in ranges {
        let (min, max) = min_max(range);

        for id in min..=max {
            if !is_valid_part_2(&id.to_string()) {
                total += id;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_10() {
        assert_eq!(is_valid_part_1("10"), true)
    }

    #[test]
    fn test_is_valid_980() {
        assert_eq!(is_valid_part_1("980"), true)
    }

    #[test]
    fn test_is_invalid_11() {
        assert_eq!(is_valid_part_1("11"), false)
    }

    #[test]
    fn test_bruteforce_solution_part_1_sample_input() {
        assert_eq!(
            bruteforce_solution_part_1(include_str!("sample_input.txt")),
            1227775554
        )
    }

    #[test]
    fn test_parts_are_equal_true_12341234() {
        assert_eq!(parts_are_equal("12341234", 4), true)
    }

    #[test]
    fn test_parts_are_equal_true_1212121212() {
        assert_eq!(parts_are_equal("1212121212", 2), true)
    }

    #[test]
    fn test_parts_are_equal_false_1011() {
        assert_eq!(parts_are_equal("1011", 2), false)
    }

    #[test]
    fn test_solution_part_2_sample_input() {
        assert_eq!(
            bruteforce_solution_part_2(include_str!("sample_input.txt")),
            4174379265
        )
    }
}
