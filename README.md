[English](README.md) | [ไทย](README.th.md)

# Learn Rust

A hands-on Rust review repo: one numbered folder per topic, short runnable examples, and CLI projects that tie the concepts together.

## Overview

This repo is a learning path, not a library. Each of the `01`–`17` folders is a self-contained lesson with its own `main.rs` and a small `README.md` containing per-lesson notes (in Thai and English). The early folders (`01`–`15`) compile with plain `rustc` and use only the Rust standard library. The last two (`16`, `17`) are Cargo projects that pull in `reqwest` and `serde` to talk to public HTTP APIs.

The examples emphasize the things that make Rust different from JavaScript/TypeScript — ownership, borrowing, and error handling — and translate familiar JS idioms into their Rust equivalents.

## What you'll learn

### Part 1 — Language fundamentals (`01`–`09`)

| # | Lesson | Key ideas |
|---|--------|-----------|
| 01 | [Variables](01-variables) | `let` immutability without `mut`, `const` needs a type and cannot be reassigned, `shadowing` re-declares a name (can change its type) |
| 02 | [Data Types](02-data-types) | `i32/u64/f64/bool/char`, destructuring `tuple`s, fixed-size `array`, owning `String` vs borrowed `&str` |
| 03 | [Operators](03-operators) | Integer division truncates, no `++`/`--` (use `+= 1`), `==` compares by value directly (no JS `===`) |
| 04 | [Conditions](04-conditions) | `if` is an expression that returns a value (replaces the ternary), `match` replaces `switch` and must be exhaustive (`_` is the default arm) |
| 05 | [Loops](05-loops) | `loop` runs forever and can return a value via `break value`, ranges `0..5` (exclusive) vs `0..=5` (inclusive), `enumerate()` gives the index |
| 06 | [Functions](06-functions) | The last line without `;` is the return value, closures replace arrow functions, no default parameters (use `Option` + `unwrap_or`) |
| 07 | [Vectors](07-vectors) | `Vec` is a growable array, `vec![]` builds one with values, out-of-bounds indexing panics (use `.get()` for an `Option`), copy with `.clone()` |
| 08 | [Structs](08-structs) | `struct` replaces the object, `impl` adds methods (`&self` is `this`), `..base` copies the remaining fields (needs `clone` since `String` is not `Copy`) |
| 09 | [Iterators](09-iterators) | `map`, `filter`, `find`, `fold`/`sum` — iterators are lazy, nothing runs until you `.collect()` |

### Part 2 — Rust's core ideas (`10`–`12`)

| # | Lesson | Key ideas |
|---|--------|-----------|
| 10 | [Ownership](10-ownership) | Assigning a `String` moves ownership so the old variable is dead, small types like `i32` are `Copy` and don't move, `.clone()` makes an explicit deep copy |
| 11 | [Borrowing](11-borrowing) | Many `&` readers are allowed, only one `&mut` writer with nothing else alive, borrows expire after their last use |
| 12 | [Error Handling](12-error-handling) | `Option` replaces null (`unwrap_or`), `Result` replaces throw (`match`/`?` propagate to the caller), `.expect()` carries a message but panics on failure |

### Part 3 — Practical CLI projects (`13`–`17`)

| # | Project | What it does | Highlights |
|---|---------|--------------|------------|
| 13 | [mini-project-text-stats](13-mini-project-text-stats) | Takes a filename and counts lines / words / characters (no argument counts a built-in sample text) | `env::args`, `fs::read_to_string`, `lines().count()`, `split_whitespace`, `chars` |
| 14 | [todo-cli](14-todo-cli) | Todo list persisted to `todos.txt` (instead of localStorage): add / list / mark done / delete | `env::args`, `fs::read`/`write`, `split_once`, `filter_map` |
| 15 | [product-search-cli](15-product-search-cli) | In-memory search and category filter; the query filters names, an optional second argument filters by category | `filter` + `contains`, dedupe categories with a `contains` check, `to_lowercase` for case-insensitive match |
| 16 | [weather-cli](16-weather-cli) | Enter a city → geocode to coordinates → fetch current weather via Open-Meteo (no API key needed) | `reqwest::blocking` (no async), `.query()` URL-encodes for you, `error_for_status`, `serde` JSON parsing |
| 17 | [github-profile-viewer](17-github-profile-viewer) | Enter a GitHub username → see followers / repo count plus the 5 most recently updated repos with star counts | Chained requests, 404 check via `error_for_status`, GitHub requires a `User-Agent`, `Option` tolerates missing fields |

## Tech stack

- **Language:** Rust (standard library only for lessons `01`–`15`).
- **Compiler:** plain `rustc` for `01`–`15`; Cargo for `16`–`17`.
- **Editions:** `--edition 2021` when invoking `rustc` directly; the Cargo projects declare `edition = "2024"`.
- **External crates (`16`, `17` only):**
  - `reqwest` `0.12` with the `blocking` and `json` features — HTTP client.
  - `serde` `1` with the `derive` feature — JSON deserialization.

## Project structure

```
learn-rust/
├── 01-variables/                 # one folder per topic
├── 02-data-types/
├── 03-operators/
├── 04-conditions/
├── 05-loops/
├── 06-functions/
├── 07-vectors/
├── 08-structs/
├── 09-iterators/
├── 10-ownership/
├── 11-borrowing/
├── 12-error-handling/
├── 13-mini-project-text-stats/   # plain rustc, each has README.md + main.rs
├── 14-todo-cli/
├── 15-product-search-cli/
├── 16-weather-cli/               # Cargo project
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs
├── 17-github-profile-viewer/     # Cargo project
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs
└── README.md
```

Each lesson folder `01`–`15` contains a `README.md` (notes for that topic) and a single `main.rs`. The Cargo projects `16` and `17` instead have a `Cargo.toml` / `Cargo.lock` and put the source in `src/main.rs`.

## Getting started

### Prerequisites

- A Rust toolchain (`rustc` + `cargo`). This repo was developed and tested with `rustc 1.99.0` / `cargo 1.99.0`.
- An internet connection for the first run of the Cargo projects (`16`, `17`), to download dependencies. The examples themselves also call public APIs at runtime.

### Running the single-file examples (`01`–`15`)

Enter a lesson folder, compile, and run:

```bash
rustc --edition 2021 main.rs -o main && ./main
```

On Windows:

```powershell
rustc --edition 2021 main.rs -o main && .\main.exe
```

### Running the Cargo projects (`16`–`17`)

From inside the project folder:

```bash
cargo run -- "Bangkok"     # 16-weather-cli
cargo run -- torvalds      # 17-github-profile-viewer
```

Useful Cargo commands:

```bash
cargo build          # compile without running
cargo run -- <args>  # compile and run with arguments
cargo check          # fast type/borrow check
cargo fmt            # format the code
cargo clippy         # lint
```

## Usage

- **Text stats** — `./main main.rs` counts the file itself; `./main` counts the built-in sample text.
- **Todo** — `./main add "read Rust"`, then `./main list`, `./main done 1`, `./main del 1`. Tasks are stored in `todos.txt` in the working directory and persist between runs.
- **Product search** — `./main` lists everything, `./main laptop` filters by name, `./main "" electronics` filters by category only.
- **Weather** — `cargo run -- "Bangkok"` prints a city's current temperature and weather code.
- **GitHub viewer** — `cargo run -- torvalds` prints the profile summary and the latest repos.

`16` and `17` default to `"Bangkok"` and `torvalds` respectively when no argument is supplied.

## Notes

- Lessons `01`–`15` use **only the standard library** and do not need Cargo or network access.
- The Cargo projects (`16`, `17`) need internet on the first run to fetch `reqwest` and `serde`; subsequent builds use the local cache.
- No API keys or credentials are required. Open-Meteo and the default GitHub API endpoints are public and need no authentication.
- The compiled binary (`main` / `main.exe`), Cargo's `target/` directory, and the generated `todos.txt` are gitignored and intentionally not committed.
- Source comments and per-lesson README notes are written in Thai; the notes are also summarized in English in each folder's README.
