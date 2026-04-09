# Rust Fundamentals Exercises

This repository contains a collection of exercises designed to learn and practice the fundamental concepts of the Rust programming language. 

The project is structured as a library (`lib.rs`) with multiple modules, separating definitions from the main execution loop and integration tests.

## 📝 Features & Exercises

### 1. String Transposition
A function that takes a reference to a vector of strings and transposes them. The output vector strings are formed by grouping the *n-th* characters of each input string. The output strings' length is bounded by the shortest string in the input vector.

### 2 & 4. Rational Numbers & Operator Overloading
A module implementing rational numbers (`Rational`).
* **Core Logic:** Fractions are automatically reduced to their lowest terms upon creation using the Greatest Common Divisor (GCD).
* **Methods:** Sum, product, inverse, and comparison.
* **Trait Implementation:** Standard library traits are heavily used here to make the struct behave like primitive numbers. Includes custom implementations for `std::ops::Add`, `std::ops::Mul`, `std::fmt::Display`, and `std::cmp::PartialOrd`, alongside derived traits like `Debug` and `PartialEq`.

### 3. Grid Survival Game
A 2D terminal-based game simulation.
* **The Grid:** An `n x n` matrix surrounded by walls. Inner cells can be empty, contain food (increases strength), or poison (decreases strength).
* **The Player:** Starts with a given strength and a random direction (Up, Down, Left, Right). 
* **Mechanics:** Each turn, a coin is flipped. Heads means the player keeps moving in the current direction; Tails means the player picks a new random direction. Hitting a wall makes the player bounce in the opposite direction.
* **Win/Loss:** The game ends in victory if the player survives for a maximum number of moves, or in defeat if their strength drops to 0 or below.

### 5. Vending Machine (State Design Pattern)
An object-oriented approach to a Vending Machine using Rust's **Trait Objects** to implement the State Pattern.
* **Context:** The `DistributoreAutomatico` holds an inventory (`HashMap<String, i32>`), the currently selected product, and its current State.
* **States:** Transitions between `InAttesaCarta`, `CartaAccettata`, `SelezionatoProdotto`, `ProdottoEsaurito`, and `ProdottoDisponibile`.
* **Actions:** Depending on the current state, actions like inserting a card, selecting a product, confirming, or canceling will have different behaviors and outputs.

## 🚀 How to Run

To run the main executable (if available, e.g., for the Game module):
```bash
cargo run
```
## 🧪 Testing
All modules are strictly tested using Rust's built-in testing framework. Tests are separated into their own directory (tests/).

To run the entire test suite:

```Bash
cargo test
```
