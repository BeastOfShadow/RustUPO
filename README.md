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

### 6. Family Tree (Shared Ownership & Interior Mutability)
A genealogy model that explores Rust's smart pointers for building graph-like, mutable, cyclic data structures.
* **The Nodes:** Each `Person` holds a name plus optional references to a `mother`, `father`, and a vector of `children`. Shared ownership across the tree is handled with `Rc<RefCell<Person>>` — `Rc` allows multiple owners (a person is referenced both as a parent's child and as its own node), while `RefCell` enables interior mutability so relationships can be wired up after creation.
* **The Collection:** `People` wraps a vector of persons and exposes the registry API: `add_person` (rejects duplicate names via `Result`), `search_person`, `set_mother`, `set_father`, and `print` (falls back to "unknown" when a parent is missing).
* **Traversal:** `descendants` walks the tree recursively, using a seen-list guard to ensure no name is ever produced twice.

### 7. English Auction (Concurrency: Threads, Channels & Shared State)
A concurrent implementation of the English Auction protocol where each actor is its own thread.
* **Actors:** One auctioneer thread plus `n` participant threads, each spawned independently.
* **Message Passing:** Communication runs entirely over `mpsc` channels — participants send `PartecipantMessage` (`Ready`, `Bid`, `Exit`) to the auctioneer, while the auctioneer broadcasts `AuctioneerMessage` (`Start`, `NewPrice`, `Winner`) back to every active participant.
* **Protocol Flow:** After all participants signal `Ready`, the auctioneer announces the product and starting price. Each round, participants either bid above the current price or drop out; every valid raise is broadcast to the remaining bidders. The auction ends once all participants have exited.
* **Outcome:** The item is sold only if the final price clears the `reserve_price`. The result (product, final price, winning participant) is written exactly once at the end into an `Arc<Mutex<AuctionOutcome>>`, the single piece of shared state read by participants — keeping with the rule that channels drive the protocol and shared memory is used only for the final result.

## 🚀 How to Run

The main executable (`main.rs`) currently runs the **English Auction** simulation (Exercise 7). The Game module (Exercise 3) and earlier `main` setups are kept as commented-out blocks you can swap back in.
```bash
cargo run
```
## 🧪 Testing
All modules are strictly tested using Rust's built-in testing framework. Tests are separated into their own directory (tests/).

To run the entire test suite:

```Bash
cargo test
```
