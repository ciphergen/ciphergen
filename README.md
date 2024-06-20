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
ciphergen generate digits 6 10
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
ciphergen generate passphrase 6 10
```

## Usernames

To generate a simple username (alternating vowels and consonants) of six characters:

```sh
ciphergen generate username simple 6
```

A batch of 10 simple usernames of eight characters:

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

Note that, in the case of complex usernames, the first numeric argument refers to the number of *syllables*, not *characters*, to be generated.

## Analysis

CipherGen supports a binary analysis feature which will provide certain relevant data about an arbitrary blob of data.

To read and analyze a file on the local filesystem, pass the path to the file as a positional argument to the `analyze` command. To read and analyze bytes from STDIN instead, leave the positional argument blank.

For example, to analyze the following string:

```bash
echo -n "All science is either physics or stamp collecting." | ciphergen analyze
```

Which will produce the following output:

```
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| Size         | 50 B                                                                                                                             |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| Entropy (Sh) | 3.866957512688445                                                                                                                |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| Entropy (So) | 24.16848445430278                                                                                                                |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| MD5          | 5d9a73410a005913e6b48599e528408d                                                                                                 |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA1         | 3cae2d09b2ce5a224cff61ce2e78c2e3d67d2df0                                                                                         |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA2-256     | bd383f7bbe6b1a37f7c7446036582a3452e631760779d50fccdd76d9b2df83a0                                                                 |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
| SHA2-512     | 09cef94efd015c8dbaf2991acb7121cd16f1106f03192c5c32b55deedf9480c41e3939bd82b3549d7f9bb65b835cdae087774630da1d8db6fcddf551da69e175 |
+--------------+----------------------------------------------------------------------------------------------------------------------------------+
```

# Visualization

CipherGen can create visualizations of arbitrary binary data.

To create a visualization of a file on the local filesystem, pass the path to the file as a positional argument to the `visualize` command. To create a visualization from STDIN instead, leave the positional argument blank.

For example, to create a visualization of the included `audio.wav` file and save the resulting PNG image to `audio.png`:

```bash
ciphergen visualize example/audio.wav -o audio.png
```

## Samples

Here are some sample images to demonstrate the visualization function:

```txt
Algorithm: SHA256
Hash:      D74EB5AAAB056842925FC90719EE6495EBF2CCFBEDBA90C09D9ABA4E8DB014A1
Path:      image/audio.png
```

![audio.wav](image/audio.png)

```txt
Algorithm: SHA256
Hash:      742B0A7B9D95B996475C02FCC56E82C4551D02FA341B41B3CD031DF2AA81C84C
Path:      image/columns.png
```

![columns](image/columns.png)

```txt
Algorithm: SHA256
Hash:      A661F23662E14A1115D103BC0D9D302D1D797040B244B4C6D4195392D241A9E3
Path:      image/ones.png
```

![ones](image/ones.png)

```txt
Algorithm: SHA256
Hash:      BF7D01D473C3B7420B72954F987E164012251102652EC63552214076CFD54659
Path:      image/random.png
```

![random](image/random.png)

```txt
Algorithm: SHA256
Hash:      E0A452FACE6C762D33ECDC7ADC704E6FCB31CC495AAF78B9F870DEE05E70C549
Path:      image/rows.png
```

![rows](image/rows.png)

```txt
Algorithm: SHA256
Hash:      A938F758B2388AF47A30D69209D5946C403CEACCF09177DB910EC5927110C94B
Path:      image/zeros.png
```

![zeros](image/zeros.png)

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
