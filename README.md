# LearningRust 🦀

A collection of small Rust projects and experiments aiming to help me learn the Rust programming language.

---

## 🚀 Motivation

I started this repository as part of my journey to learn Rust.  
By working on small, standalone programs, I can explore different aspects of the language — from basic syntax to file I/O, error handling, testing and more.  
Each example is simple and self-contained, making it easier to understand and revisit later.

---

## 📁 What’s inside

| Example / Module | Description |
|------------------|-------------|
| `guessing_game`   | A simple number guessing game — practice with user input, loops, conditionals. |
| `mini_calculator` | A minimal command-line calculator — practice parsing input, basic math, error handling. |
| `file_reader`     | Read and display contents of a file — practice file I/O in Rust. |
| `simple_curl`     | A basic HTTP fetch utility — exploring networking / external requests. |
| `temperature_converter` | Command-line tool converting between Celsius, Fahrenheit etc. — practice with data handling and formatting. |
| `todo_list`       | A command-line to-do list — practice data persistence / structuring programs. |
| `test_save`       | Examples related to saving data and testing — practice Rust’s testing and serialization/IO. | 

---

## 🧑‍💻 How to run / build

Make sure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

Then from the root of this repo:

```bash
# To compile and run a specific example, e.g. guessing_game:
cd guessing_game
cargo run

# Or run tests (if any) across all modules:
cargo test
