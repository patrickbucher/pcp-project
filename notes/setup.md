# Setup

A full Rust setup contains the following tools:

- `rustc`: the rust compiler
- `cargo`: package management and build system
- `rustdoc`: documentation viewer
- `rustfmt`: code formatter

## Prerequisites

- macOS/Linux: a linker and optionally a C compiler (`gcc` or `llvm` are common options)
- Windows: [C++ Build Tools for Visual Studio (2013 or Later)](https://www.visualstudio.com/downloads)

## macOS and Linux: Using `rustup`

```bash
$ curl https://sh.rustup.rs -sSf | sh 
```

The `curl` parameters are needed to prevent `sh` receiving any garbage, and they mean:

- `-s`: silent
- `-S`: show errors
- `-f`: fail silently

Open a new shell (or type `source $HOME/.cargo/env`) to update the environment
variables, then check the installation:

```bash
$ rustc --version
rustc 1.30.1 (1433507eb 2018-11-07)
```

Update:

```bash
$ rustup update
```

Uninstall:

```bash
$ rustup self uninstall
```

## macOS and Linux: Using Package Manager

Homebrew (macOS):

```bash
$ brew install rust
```

aptitude (Debian, Ubuntu):

```bash
# aptitude install rustc cargo rust-doc
```

pacman (Arch Linux):

```bash
# pacman -S rust rust-docs
```
