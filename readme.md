# Gitignore Checker

## Synopsis

This project was undertaken solely to comprehend the ignore mechanism of Git. As it presented a welcome opportunity to refresh the author's acquaintance with Rust, he embraced the occasion.

**Note**: This project is not yet complete; its development was carried forward only so far
as the author felt at ease with the rules governing Git’s ignoring mechanism.

## Installation

### Requirements

- Rust 2024

### Setup

1. Navigate to a working directory of your choice, then clone the repository and enter it:

   ``` sh
   git clone https://github.com/kvn-dtrx/gitignore-checker.git &&
       cd gitignore-checker
   ```

2. Compile the project:

   ``` sh
   cargo build
   ```

## Usage

The output of the following command contains a step-by-step explanation of whether a gitignore file residing at `<path/to/gitignore>` would ignore the hypothetical path `<path/to/check/against>`:

``` sh
cargo run -- <path/to/gitignore> <path/to/check/against>
```

## Colophon

**Author:** [kvn-dtrx](https://github.com/kvn-dtrx)

**License:** [MIT License](license.txt)
