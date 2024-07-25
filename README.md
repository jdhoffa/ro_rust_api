# Read-Only Rust API

This is a simple read-only API built with Rust and Rocket that serves data from different CSV files in JSON format, via two routes.

## Project Structure

- `Cargo.toml`: Project configuration and dependencies.
- `src/main.rs`: Main source code.
- `data/`: Directory containing `.csv` datasets to be served by the API.

## Prerequisites

- You must install Rust and Cargo to build and run this project. You can follow the instructions [here](https://www.rust-lang.org/tools/install).

## Running the API (Rust)

To build and run the project:

  ```bash
  cargo run
  ```
The server will start on http://localhost:8000.
You can access the canonical "Iris" dataset at http://localhost:8000/data/iris.
You can access the "Boston Housing" dataset at http://localhost:8000/data/boston.

You can also download the data to your computer using `curl`:

  ```bash
  curl http://localhost:8000/data/iris
  ```

## Running the API (Docker)

You can also build and run the API using Docker and `docker compose`:

    ```bash
    docker compose up --build
    ```
The server will start on http://0.0.0.0:8000. You can access the data at http://0.0.0.0:8000/data/iris.
