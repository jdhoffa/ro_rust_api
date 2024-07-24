# Read-Only Rust API

This is a simple read-only API built with Rust and Rocket that serves data from a CSV file in JSON format.

## Project Structure

- `Cargo.toml`: Project configuration and dependencies.
- `src/main.rs`: Main source code.
- `data/iris.csv`: CSV file containing the canonical Iris dataset.

## Prerequisites

- You must install Rust and Cargo to build and run this project. You can follow the instructions [here](https://www.rust-lang.org/tools/install).

## Setting Up the Project

To build and run the project:

  ```bash
  cargo run
  ```
The server will start on http://localhost:8000. You can access the data at http://localhost:8000/data.
