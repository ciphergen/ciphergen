# Introduction

CipherGen is a native Rust program that provides a command-line interface (CLI) for generating cryptographic data, such as secret keys, pronounceable usernames and PINs.

# Usage

CipherGen is currently capable of generating the following types of random data:

- Raw Bytes
- Hexadecimal Bytes
- Base64 Bytes
- Numbers
- Digits (PIN)
- Alphanumeric Passwords
- Passphrases
- Usernames

CipherGen can also analyze binary data and produce informational reports.

## Bytes

To write a sequence of 256 raw bytes to STDOUT:

```sh
ciphergen generate bytes 256
```

For the same number of bytes, encoded as hexadecimal:

```sh
ciphergen generate hex 256
```

Or, as Base64:

```sh
ciphergen generate base64 256
```

Please bear in mind that the numeric positional argument always refers to the number of *bytes*, not *characters*, to be generated.

## Numbers

To generate a random number between one and one thousand:

```sh
ciphergen generate number 1 1000
```

For a batch of ten numbers between ten and ten thousand:

```sh
ciphergen generate number 10 10000 10
```

## PINs

To generate a four-digit PIN:

```sh
ciphergen generate digits 4
```

For a batch of ten, six-digit PINs:

```sh
ciphergen generate digits 4 10
```

## Passwords

To generate a single eight-character password:

```sh
ciphergen generate password 8
```

To generate a batch of ten, sixteen-character passwords:

```sh
ciphergen generate password 16 10
```

## Passphrases

To generate a single four-word passphrase:

```sh
ciphergen generate passphrase 4
```

To generate a batch of ten, six-word passphrases:

```sh
ciphergen generate passphrase 4 10
```

## Usernames

To generate a simple username (alternating vowels and consonants) of six characters:

```sh
ciphergen generate username simple 6
```

A batch of 10 simple usernames of 8 characters:

```sh
ciphergen generate username simple 8 10
```

To generate a complex username (composed of syllables) of 3 syllables:

```sh
ciphergen generate username complex 3
```

Or, for a batch of 10 complex usernames of two syllables:

```sh
ciphergen generate username complex 2 10
```

Note that, in this case, the first numeric argument refers to the number of *syllables*, not *characters*, to be generated.

## Analysis

To analyze a string:

```sh
ciphergen analyze "All science is either physics or stamp collecting."
```

Which will produce the following output:

```
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| Size    | 50 B                                                                                                                             |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| Entropy | 3.8669575126884443                                                                                                               |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| MD5     | 5d9a73410a005913e6b48599e528408d                                                                                                 |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA1    | 3cae2d09b2ce5a224cff61ce2e78c2e3d67d2df0                                                                                         |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA256  | bd383f7bbe6b1a37f7c7446036582a3452e631760779d50fccdd76d9b2df83a0                                                                 |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA512  | 09cef94efd015c8dbaf2991acb7121cd16f1106f03192c5c32b55deedf9480c41e3939bd82b3549d7f9bb65b835cdae087774630da1d8db6fcddf551da69e175 |
+---------+----------------------------------------------------------------------------------------------------------------------------------+
```

To read and analyze bytes from STDIN instead, leave the positional argument blank.

# Contributing

Contributions are welcome! Feel free to open an issue or pull request.

## Branches

- `dev` <br/> All pull requests are merged into this branch. May be pushed.
- `main` <br/> Default branch. May not be pushed or force pushed.
- `stable` <br/> Points to the ref of the latest Git tag with a name matching the following regular expression: <br/> `/^v?([0-9]+)\.([0-9]+)\.([0-9]+)$/` <br/> May not be pushed or force pushed.

## Building

```sh
cargo build
```

## Testing

```sh
cargo test
```
