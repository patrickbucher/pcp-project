# Tooling

## Cargo

Create a new project (binary applicatoin) `hello` with `.gitignore` file:

    cargo new --bin --vcs git hello

Build the program (debug):

    cargo build

Build the program (release)

    cargo build --release

Run the program directly:

    cargo run

Check the source (without creating a binary):

    cargo check

Update dependencies:

    cargo update

Show documentation for the current project in browser:

    cargo doc --open
