# Safe Password Generator

Safe Password Generator is a simple command-line tool written in Rust to generate random passwords. The generated passwords contain upper and lower case letters as well as numbers.

## Features

- Generates strong, random passwords.
- Easy to use from the command line.
- Lightweight and fast.

## Installation

To install Safe Password Generator, you need to have Rust and Cargo installed on your machine. If you don't have them installed, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

1. Clone the repository:

```sh
git clone https://github.com/yourusername/safe-password-generator.git
```

2. Navigate to the project directory
```sh
cd safe-password-generator
```

3. Install the tool using Cargo
```sh
cargo install --path .
```

## Usage

Once installed, you can generate a random password by simply running:

```sh
safe-password-generator
```

The following command line arguments are offered:
- `-l` or `--length`: Any positive integer. Defines the length of the generated password.
- `--alphanumeric`: If passed, output will only contain alphanumeric characters.
- `-c` or `--clean`: If passed, the "Your password: " part of the output will be hidden.
