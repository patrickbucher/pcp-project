```rust
fn main() {
    println!("Rust");
    println!("===================");
    println!("")
    println!("- Ownership");
    println!("- Traits");
    println!("- Concurrency");
    println!("- Pattern Matching");
}

// Rust ist eine Systemprogrammiersprache, die 
// blitzschnell läuft, Speicherfehler vermeidet
// und Threadsicherheit garantiert. 
```

## Was ist Rust?
- __Paradigma:__ Multiparadigmen
    - (generisch, nebenläufig, funktional, imperativ, strukturiert)
- __Erscheinungsjahr:__	2010
    - erste stabile Version 2015
- __Entwickler:__ Graydon Hoare (Mozilla) 
- __Aktuelle Version:__ 1.31 (6. Dezember 2018)
- __Typisierung:__ stark, statisch, linear, Typinferenz
- __Features:__ 
    - Zero-Cost-Abstraktionen, Move-Semantiken
    - Garantierte Speichersicherheit, Threads ohne Data Races
    - Trait-basierte Generics, Pattern Matching, Typinferenz
    - Minimales Laufzeitsystem, Effiziente Schnittstelle zu C

## SP1: Ownership-Konzept
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

## SP2: Traits
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    // ...
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}
```

## SP3: Concurrency
TODO

## SP4: Pattern Matching
TODO

## Technisches Team-Fazit
TODO

## Persönliches Fazit - Patrick

- intelligenter Compiler
- gutes Tooling (`cargo`, `rustfmt`)
- dünne Standard-Library (Abhängigkeit von Libraries)
- teils gewöhnungsbedürftig (Syntax, Memory-Handling)
- zwischen Rust und Go hin und her gerissen
    - Vorteile von Rust:
        - ausgeklügeltes Typsystem
        - kein Garbage Collector
        - Performance
    - Vorteile von Go:
        - bessere Standard-Library
        - einfachere Syntax
        - _noch_ besseres Tooling
        - Google und Unix-Genies dahinter: Thompson, Pike, Kernighan (Buch)

Fazit: Ich werde mich weiter mit Rust beschäftigen.

## Persönliches Fazit - Lukas
TODO
