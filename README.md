# RustMate: A chess engine implemented in Rust

## This is a work in progress. 

An Chess engine implemented in Rust. I started this project to learn Rust and get more familiar with more advanced AI, heuristic, and search techniques for adversarial games.

> Note: The original idea was to make the game in Python along with Rust as the AI engine. That proved to not be
the best approach. Soon the API in Pyton will be replace for a API in Rust (once I get more experienced in Rust crate to create Rest applications).

#### Implemented features

- [x] The chess game is implemented, working and correctly generating valid moves;
- [x] The engine can read a FEN position and continue the match from there;
- [x] Each game position has a Zobrist Hash associated
- [x] A simple AI is implemented using the Negamax algorithm and performs under 1 sec for a depth of 4.
  - [x] Move ordering and alpha beta pruning is implemented to short the number of states to be searched
- [x] A heatmap is being used to give or remove points for a piece standing in square (example: usually, the king should not be in the middle of the board at the opening and mid game phases)


#### Missing features I want to implement

- [ ] Iterative search;
- [ ] Search optimizations:
  - [ ] Go deeper in search when a leaf is supposed to be good (example: would be interesting to go deeped after finding a check, who knows if it's going to be mate);
  - [ ] Optimize search using a Transposition table (Zobrist hashing);
- [ ] Evaluation optimizations
  - [ ] Killer moves;
  - [ ] King safety;
  - [ ] Center control;
  - [ ] Unmoved pieces
- [ ] Migrate API from Python to Rust (once I get more experienced in Rust crate to create Rest applications);
- [ ] Provider better infos to create visuals on what the AI is thinking;


Author: Thiago Buarque