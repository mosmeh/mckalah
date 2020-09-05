# mckalah

[![build](https://github.com/mosmeh/mckalah/workflows/build/badge.svg)](https://github.com/mosmeh/mckalah/actions)

Monte Carlo tree search player for [Kalah](https://en.wikipedia.org/wiki/Kalah)

## Installation

Clone this repository and run:

```sh
cargo install --path .
```

## How to play

`mckalah` with no option will start human vs. MCTS game, where the human is a first player.

```sh
mckalah
```

Pits are selected with numbers 1-6.

```
        3  3  3  3  3  3
        0              0
        3  3  3  3  3  3

Select: 1  2  3  4  5  6
```

`human`, `random` and `mcts` are available as player types.

```sh
# MCTS vs. human (human is a second player)
mckalah mcts human

# Start with 4 stones in each pit
mckalah -n 4

# random vs MCTS with a timeout of 0.1s
mckalah random mcts -t 0.1
```

## Options

```
USAGE:
    mckalah [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <n>                     n stones in each pit [default: 3]
    -t, --timeout <timeout>    Timeout for Monte Carlo tree search in seconds [default: 1]

ARGS:
    <first>     One of human, random, or mcts [default: human]
    <second>     [default: mcts]
```
