# polite

[![crates.io: polite](https://img.shields.io/crates/v/polite.svg?label=polite)](https://crates.io/crates/polite)
[![crates.io: polite-cli](https://img.shields.io/crates/v/polite-cli.svg?label=polite-cli)](https://crates.io/crates/polite-cli)
[![MIT licensed](https://img.shields.io/crates/l/polite)](https://github.com/lmmx/polite/blob/master/LICENSE)


A **rusqlite × Polars** bridge for ergonomic conversion between SQLite databases and Polars DataFrames.

## Project Structure

This workspace contains multiple interconnected crates:

### Rust Libraries

- **[polite/](https://github.com/lmmx/polite/blob/master/polite)** – Core library for rusqlite ↔ Polars bridging
- **[polite-cli/](https://github.com/lmmx/polite/blob/master/polite-cli)** – Command-line tool for running SQL and exploring results as Polars DataFrames

## Features

- Convert SQLite query results into Polars `DataFrame`s
- Insert Polars `DataFrame`s into SQLite tables
- Safe, idiomatic Rust APIs
- Companion CLI for testing and scripting

## Development

- See [CONTRIBUTING.md](https://github.com/lmmx/polite/blob/master/CONTRIBUTING.md)
  and [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License. See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
