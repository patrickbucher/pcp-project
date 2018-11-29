# Setup

A full Rust setup contains the following tools:

- `rustc`: the rust compiler
- `cargo`: package management and build system
- `rustdoc`: documentation viewer
- `rustfmt`: code formatter

## Prerequisites

- macOS/Linux: a linker and optionally a C compiler (`gcc` or `llvm` are common options)
- Windows: [Microsoft Visual C++ Redistributable for Visual Studio 2017](https://www.visualstudio.com/downloads)
	- [64 Bit](https://aka.ms/vs/15/release/VC_redist.x64.exe)
	- [32 Bit](https://aka.ms/vs/15/release/VC_redist.x86.exe)

## macOS and Linux: Using `rustup`

Open a terminal and type:

```bash
$ curl https://sh.rustup.rs -sSf | sh 
```

Then follow the instructions.

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

## Windows: Using `rustup-init`

1. Download `rustup-init.exe` from [win.rustup.rs](https://win.rustup.rs/)
2. Execute the binary and follow the instructions.
3. Take a long nap, for [the installation takes much longer](https://github.com/rust-lang/rustup.rs/issues/763) than on macOS and Linux.
4. Verify the installation:

```cmd
> rust --version
rustc 1.30.1 (1433507eb 2018-11-07)
```
