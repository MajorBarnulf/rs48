# rs48

## Description

It is a game of 2048 that plays in the terminal as a TUI, it has a lot of
configurability and implements a few types of AI.

## Usage

Compiled with the [rustup toolchain](https://rustup.rs).

### Installing
```sh
cargo install rs48
```

alternatively, you can clone this repo and build it yourself.

### Building

```sh
git clone "https://github.com/MajorBarnulf/rs48.git"
cd rs48/rs48
cargo build -r
```

### Running

```sh
cargo run -r
```

### Help
```s
Usage: rs48 [OPTIONS]

Options:
  -s, --size <SIZE>
          size of the grid on which the game is played [default: 4]
  -w, --spawn <SPAWN>
          number of tiles that will spawn on the grid each turn [default: 1]
      --no-clear
          disable clearing the terminal to refresh the screen
  -k, --display-skips <DISPLAY_SKIPS>
          skips the refresh of that many turns, allow AIs to play faster [default: 0]
  -d, --delay <DELAY>
          delay in ms to add between each turns [default: 0]
  -c, --controller <CONTROLLER>
          the controller to use for the game [default: player]
      --color-seed <COLOR_SEED>
          sets a seed for the color pattern, 0 for random, default is 35 [default: 35]
  -h, --help
          Print help information
  -V, --version
          Print version information
```