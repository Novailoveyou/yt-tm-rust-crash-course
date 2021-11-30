# Rust Crash Course | Rustlang

## About

This is me following Brad Traversy's [youtube video](https://youtu.be/zF34dRivLOw)

Year 2021

## Official Rust website

[Vist Rust Website](https://www.rust-lang.org/)

## CLI

Install Rust on Unix-like OS

```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Get Rust version

```zsh
rustup --version
```

Check for Rust updates and update if needed

```zsh
rustup update
```

Get Rust compiler version

```zsh
rustc --version
```

Get Rust package manager version

```zsh
cargo --version
```

Compile Rust (.rs) hello.rs file

```zsh
rustc hello.rs
```

Run compiled executable

```zsh
./hello
```

Init project with Cargo (create new project in a folder named *hello*)

```zsh
cargo new hello
```

Init project with Cargo in current folder

```zsh
cargo init
```

Compile and run project with Cargo

```zsh
cargo run
```

Compile project with Cargo

```zsh
cargo build
```

Build project with Cargo for production

```zsh
cargo build --release
```

## Things I learned

### Rust CLI Utils

`rustup` - version manager

`rustc` - compiler

`cargo` - package manager

### Path to executable named *filename* compiled with Cargo

./target/debug/filename

### Path to production file named *filename* built with Cargo

./target/release/filename
