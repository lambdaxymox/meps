# Machine Epsilon

## Introduction
**Machine Epsilon** is a shell program that calculates the error bounds on the representation of a floating point number at 32 bits or 64 bits on your system.

## Usage
The program `meps` is a shell program. To use it from your shell, enter
```bash
meps <floating point number>
```
where `<floating point number` is a string representing a floating point number.

## Installation
Install `meps` by forking this repository and entering
```bash
cargo install --path .
```
using `cargo`. Alternatively, install it directly from the GIT URL.
```bash
cargo install --git https://github.com/stallmanifold/meps
```
