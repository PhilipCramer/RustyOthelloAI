# Rusty Othello AI

AI capable of playing the game Othello implemented in Rust.

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## Introduction
Rusty Othello AI is an implementation of an artificial intelligence capable of playing the game Othello (also known as Reversi) using Rust. The AI uses Monte Carlo Tree Search (MCTS) to determine the best moves.

## Installation
To install and run the Rusty Othello AI, you need to have Rust installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Clone the repository to your local machine:
```sh
git clone https://github.com/PhilipCramer/RustyOthelloAI.git
cd RustyOthelloAI
```

Build the project:
```sh
cargo build --release
```

## Usage
To run the AI, execute the following command:
```sh
cargo run --release <color>
```
Replace `<color>` with either `black` or `white` to specify the AI's color.

## Project Structure
- `src/main.rs`: The main entry point of the application. It handles the game loop, command-line arguments, and interactions with the server.
- `src/mcts.rs`: Contains the implementation of the Monte Carlo Tree Search algorithm.
- `src/othello.rs`: Contains the implementation of the Othello game logic, including game state and actions.
- `Cargo.toml`: Contains the project metadata and dependencies.

## Contributing
Contributions are welcome! Please fork the repository and open a pull request with your changes.
