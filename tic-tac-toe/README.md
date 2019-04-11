# Tic Tac Toe - Rust

I was originally hoping to write this Tic Tac Toe in Go or in Elixir, but
eventually settled for Rust as it is a language that I believe will take me
very far out of my comfort zone. It is also a good way for me to test my mettle
and see how I cope with ramping up on such a language in such a short amount of
time.

## This Tic Tac Toe is not unbeatable
Rust has proven to be pretty difficult for me to get my head around, and after three
different attempts at an unbeatable algorithm (minimax twice in an OO fashion,
negamax once in an FP fashion), I'm afraid I was not able to deliver.

You can currently explore the code by checking out the `unbeatable-player` branch of this
repository. At current, I believe that the problem comes from my `calculate_score` function.
You will notice there is an `else` statement on `L173` that returns `0`. It was added in because
if statements in Rust require an else to work in most cases, and match conditions also require a
default case. However, Rust does not have a `nil` or `null` function, and I couldn't break out of
the loop or print nothing. It is not supposed to be reached by my code, but a morning of debugging
showed me that it is.

I suspect it happens when the game is still in progress, however, I am stumped as to why this happens,
given that this function should only be reached when the game is not in progress (this is defined in the
`negamax` function that calls `calculate_score`.

"But Marion, where are your tests?!" I hear you say. My Negamax implementation was written on Wednesday,
in a fit of panic and I thought I would lose time trying to TDD Negamax. My plan was to translate my Clojure
Negamax, and write the tests when I had all my functions. I used manual testing as a gauge of whether I was
headed in the right direction, and was hit hard by the dreaded Borrow Checker, which led me down a few
referential rabbit holes (it does that thing where it tells you to consider using a reference when something
doesn't work, and you do that and then it complains that you need to add lifetimes and then you try to do that
and not only does it not work, you wasted an hour doing things you now realise you now need to undo).

By then, it was 7pm and my brain had melted. So yeah, I don't have tests and it was a mistake.
I tried to salvage it this morning, and decided to focus on my presentation instead, as I believe
that despite not being unbeatable, the current code shows my design and architecture decisions.

## Requirements
- Rust - I use `rustc 1.33.0 (2aa4c46cf 2019-02-28)` on this project, and I
followed the installation instructions given on the
[Rust website](https://www.rust-lang.org/tools/install).

## Running the programme
- You can start the programme by running `cargo build` from the `tic-tac-toe` folder.
- You can run the tests by running `cargo test` from the `tic-tac-toe` folder.

The game is currently configured to run a Human VS Simple Computer game. This can be changed by
updating the type of the player in `main.rs`:

- `let human_player = Human::new("X".to_string());` for a Human Player
- `let computer_player = Computer::new("O".to_string());` for a Computer Player

You can change the symbol for another character (or anything else), as long as it stays
between the string's quotation marks. I.e. `"💩"` will work, but 💩 will not.

For some extra fun, you can install [Clippy](https://github.com/rust-lang/rust-clippy) and run `cargo clippy`
from the `tic-tac-toe` folder to see all the style issues my code has (I got rid of most of them).

## Features
- [x] 3x3 board
- [x] Human player (X)
- [x] Human VS Human game
- [x] Simple Computer player (O)
- [ ] Unbeatable Computer player (O)
- [x] Input validation

## Areas for improvement
- [ ] String VS &str - It would be nice to handle them elegantly using `AsRef` (you will see Clippy complain about it!)
- [ ] The game status are currently strings, they could be made into Enums
- [ ] The logic implemented to check a winner is clunky and inelegant
- [ ] I do not have integration tests
- [ ] I am also not testing I/O capture.
- [ ] I haven't been able to reproduce it consistently, but I think playing with emojis
messes up with the logic to stop the computer from overwriting the position if it's not empty.

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
is a shift to think about these concepts while coding, and ten days has not been long enough for me to master
them.

- Although there are no integration tests or I/O tests, these cases were manually tested.
These tests tend to be time-consuming when you don't have the best knowledge of the language,
and I favoured delivering a program that is unit-tested and has most of the feature requested
than an even more incomplete program.