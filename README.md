# Advent of Code 2025

This repository is a workspace for solving Advent of Code 2025 in Rust. A new crate is added for each day (e.g., `day_1`, `day_2`, …). Each day is an isolated Rust package with its own source, tests, and input files.

### Current status
- Implemented: Day 1 — Secret Entrance (both parts)
- Implemented: Day 2 — Gift Shop (both parts)
- Implemented: Day 3 — Lobby (both parts)
- Implemented: Day 4 — Printing Department (both parts)
- Implemented: Day 5 — Cafeteria (both parts)
- Implemented: Day 6 — Trash Compactor (both parts)
- Implemented: Day 7 — Trash Compactor (both parts)


---
## Repository layout
- `Cargo.toml` — Workspace manifest listing all day crates under `members`.
- `day_1/` — Crate for Day 1
  - `src/lib.rs` — Core solution and tests.
  - `src/sample_input.txt` — Sample input from the problem statement.

### Toolchain
- Rust edition: 2024
- Recommended: latest stable Rust toolchain (`rustup update`) and `cargo`

---

## How to build and test

You can run tests for all days (or individual days) using cargo:

To run every test in the workspace
```bash
cargo test
```

To run tests just one day:
```bash
cargo test -p day_1
```

---

### Adding a new day

Each day lives in its own crate at the repository root (e.g., `day_2`, `day_3`). To add a new day:

```
cargo new day_x --lib
```

In `Cargo.toml`, add the new crate to the workspace:

```
[workspace]
resolver = "3"
members = [
    "day_1",
    "day_2",  
]
```

---



### Inputs

Sample inputs are located in the `src` directory of each day crate.

