//! Advent of Code 2025 — Day 1: Secret Entrance

struct Dial {
    /// The current position of the dial. This value should always
    /// be between `0` (inclusive) and `size` (exclusive).
    current_position: u64,

    /// The size of the dial
    size: u64,
}

/// A rotation direction on the dial.
enum Direction {
    Left,
    Right,
}

/// A single rotation command with a direction and number of steps.
struct Command {
    direction: Direction,
    steps: u64,
}

impl Dial {
    /// Apply a rotation command to the dial, updating `current_position`.
    ///
    /// The dial wraps around using modulo arithmetic, so positions remain
    /// within `0..size`.
    fn move_position(&mut self, command: &Command) {
        let steps = command.steps % self.size;

        self.current_position = match command.direction {
            Direction::Right => (self.current_position + steps) % self.size,
            Direction::Left => (self.current_position + self.size - steps) % self.size,
        }
    }

    /// Calculates how many times the dial will hit `0` for a given command,
    /// without actually moving the dial.
    ///
    /// We count every intermediate click that lands on `0` while rotating, and
    /// also the final click if it ends at `0`.
    ///
    /// Notes:
    /// - This function does not mutate the dial; it is a pure counter.
    /// - Beware large step counts (e.g., `R1000` from `p=50` on `N=100` yields
    ///   `10` hits of `0`).
    fn count_zeros(&self, command: &Command) -> u64 {
        match command.direction {
            Direction::Right => (self.current_position + command.steps) / self.size,
            Direction::Left => {
                let dist_to_zero = if self.current_position == 0 {
                    self.size
                } else {
                    self.current_position
                };

                if command.steps < dist_to_zero {
                    0
                } else {
                    1 + (command.steps - dist_to_zero) / self.size
                }
            }
        }
    }
}

impl Command {
    /// Create a new turn command
    fn new(direction: Direction, steps: u64) -> Self {
        Self { direction, steps }
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", value)),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Expected format: one of `L` or `R` followed by an unsigned integer.
        // Examples: "L68", "R14", "L1".
        let direction = Direction::try_from(&value[0..1])?;
        let steps = value[1..]
            .parse()
            .map_err(|_| format!("Invalid step count"))?;

        Ok(Command { direction, steps })
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            current_position: 50,
            size: 100,
        }
    }
}

/// Solve Part 1: count how many times the dial is at position `0`
/// after executing each command from the given input.
///
/// The `input` is expected to contain one command per line (e.g., `L68`, `R14`).
/// Invalid lines will cause a panic via `expect`, matching AoC-style assumptions
/// for trusted inputs.
///
/// Returns the number of times the dial points at `0` immediately after a move.
fn solution_part_1(input: &str) -> u64 {
    let mut dial = Dial::default();
    let mut password = 0;

    for line in input.lines() {
        let command = Command::try_from(line).expect("Could not read command");
        dial.move_position(&command);

        if dial.current_position == 0 {
            password += 1;
        }
    }

    password
}

/// Solve Part 2: count how many times the dial hits `0` during each rotation.
///
/// For each command, we:
/// 1) Add how many times `0` will be clicked during that rotation
///    (using `Dial::count_zeros`).
/// 2) Apply the rotation to update the dial position (`Dial::move_position`).
///
/// Returns the total count of `0` hits across all rotations.
fn solution_part_2(input: &str) -> u64 {
    let mut dial = Dial::default();
    let mut password = 0;

    for line in input.lines() {
        let command = Command::try_from(line).expect("Could not read command");

        password += dial.count_zeros(&command);
        dial.move_position(&command);
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_left() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Left, 10));
        assert_eq!(dial.current_position, 40);
    }

    #[test]
    fn test_turn_right() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Right, 10));
        assert_eq!(dial.current_position, 60);
    }

    #[test]
    fn test_turn_left_over_0() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Left, 60));
        assert_eq!(dial.current_position, 90);
    }

    #[test]
    fn test_turn_right_over_0() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Right, 60));
        assert_eq!(dial.current_position, 10);
    }

    #[test]
    fn test_turn_right_multiple_times() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Right, 210));
        assert_eq!(dial.current_position, 60);
    }

    #[test]
    fn test_turn_left_multiple_times() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Left, 210));
        assert_eq!(dial.current_position, 40);
    }

    #[test]
    fn test_turn_left_exact_on_0() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Left, 50));
        assert_eq!(dial.current_position, 0)
    }

    #[test]
    fn test_turn_right_exact_on_0() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Right, 50));
        assert_eq!(dial.current_position, 0)
    }

    #[test]
    fn test_turn_left_full_circle() {
        let mut dial = Dial::default();
        dial.move_position(&Command::new(Direction::Left, 50));
        assert_eq!(dial.current_position, 0);

        dial.move_position(&Command::new(Direction::Left, 100));
        assert_eq!(dial.current_position, 0);
    }

    #[test]
    fn test_sample_input_part_1() {
        let result = solution_part_1(include_str!("sample_input.txt"));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sample_input_part_2() {
        let result = solution_part_2(include_str!("sample_input.txt"));
        assert_eq!(result, 6);
    }
}
