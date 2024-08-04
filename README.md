# Amazing

Amazing is a game loosely based on the [Longest path problem](https://en.wikipedia.org/wiki/Longest_path_problem). It involves maximalizing the path through the graph by placing tiles on 2d board predefined with entrances, checkpoints, exits and walls.

The core of the game is written in Rust and is exposed to the web through WebAssembly. In the future API is planned for most of the game features but WebAssembly runtime will stay to offload most of the work from the server.

## Project structure

- `/apps`
  - `console` - Debug console
  - `web` - Next.js web application
- `/crates`
  - `maze-core` - Crate defining core maze structures and validations
  - `maze-runner` - Engine for maze evaluation
  - `maze-generator` - Generator of maze boards from defined presets
  - `mazer` - Webssemly binding

## WIP

The project is currently heavily in progress. Just the core of the game, WebAssembly bindings and base for a web runtime are implemented.

### Future

It is not fully decided what exact direction will Amazing go, but the current goal is a web based application with training, multiplayer, and daily challenge game modes.
