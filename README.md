# Introduction

CipherGen is a native Rust program that provides a command-line interface (CLI) for generating cryptographically-secure secret keys, such as passwords and passphrases, as well as random pronounceable usernames.

# Usage

To generate an eight-character password, use the following command:

```sh
ciphergen generate password 8
```

# Contributions

## Branches

- `dev` <br/> All pull requests are merged into this branch. May be pushed.
- `main` <br/> Default branch. May not be pushed or force pushed.
- `stable` <br/> Points to the ref of the latest Git tag with a name matching the following regular expression: <br/> `/^v?([0-9]+)\.([0-9]+)\.([0-9]+)$/` <br/> May not be pushed or force pushed.

## Build

```sh
cargo build
```

## Test

```sh
cargo test
```
