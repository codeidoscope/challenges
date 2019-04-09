# Tic Tac Toe - Rust

I was originally hoping to write this Tic Tac Toe in Go or in Elixir, but
eventually settled for Rust as it is a language that I believe will take me
very far out of my comfort zone. It is also a good way for me to test my mettle
and see how I cope with ramping up on such a language in such a short amount of
time.

## Requirements
- Rust - I use `rustc 1.33.0 (2aa4c46cf 2019-02-28)` on this project, and I
followed the installation instructions given on the
[Rust website](https://www.rust-lang.org/tools/install).

## Running the programme
- You can start the programme by running `cargo build` from the `tic-tac-toe` folder.
- You can run the tests by running `cargo test` from the `tic-tac-toe` folder.

## Features
- [x] 3x3 board
- [x] Human player (X)
- [x] Human VS Human game
- [x] Simple Computer player (O)
- [ ] Unbeatable Computer player (O)
- [x] Input validation

## Areas for improvement
- [ ] The game status are currently strings, they could be made into Enums
- [ ] The logic implemented to check a winner is very clunky and inelegant and
I would like to revisit it and apply a more functional approach
- [ ] I do not have integration tests yet
- [ ] I am also not testing I/O capture.

## Decisions and compromises made

- Rust allows for both functional and OOP implementations. I have found myself to be
picking and mixing both in places that suited me. I found that building a TTT in an OO
way was easier to follow, especially when tackling a new language, but Rust's iterators
are great and I've used `map` in several places to keep some of my functions lean.

- `mem::swap` was used to swap players as it seemed the safest way to do such an
operation without using Rust's  `unsafe` features.

- I'm relying on internal mutation in two places: In the `Board`, the `symbol` field
of the `Tile` struct, and in the `Game`, the `current_player_move` field of the `Game`
struct. I made the decision to use `RefCell` and `Cell` respectively, as it allowed me
to not mutate all of the structs, which I believe could have introduced more instability
should accidental changes be made to them. The concept is described as a "last resort" in
the Rust documentation (see [std::cell](https://doc.rust-lang.org/std/cell/)), but it is
mentioned that it can be introduced to ensure mutability _inside_ an immutable object, which
is the decision I made when implementing this.

- Some of the tests can appear oddly organised, with variables created before or after objects
are instantiated. This is because of the mutability and ownership constraints set by Rust. It
is a shift to think about these concepts while coding, and ten days has not been long enough to master
them.