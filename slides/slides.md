```rust
fn main() {
    println!("Rust");
    println!("===================");
    println!("")
    println!("- Ownership");
    println!("- Traits");
    println!("- Pattern Matching");
    println!("- Concurrency");
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

## Geschichte von Rust

- `2006..2010`: Privates Projekt vom Mozilla-Mitarbeiter Graydon Hoare
- `2010..2012`: Mozilla nimmt Rust unter seine Obhut
    - Firefox: 4.5M Zeilen C++
- `2012..2014`: Einbindung der Community, Weggang von Graydon Hoare
- `2014..2016`: Stabilisierung (Version 1.0.0), Fokus auf Libraries
- `2016..2018`: Produktiveinsatz (Servo, Dropbox), Redox, Version 1.31

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

## SP2: Traits (Interface)
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

## SP2: Traits (Extension)
```rust
pub trait Hello {
    fn hello(&self);
}

impl Hello for String {
    fn hello(&self) {
        println!("Hello");
    }
}

fn main() {
    let s = String::from("World!");
    s.hello();
}
```

## SP3: Pattern Matching (`Option` statt `null`)

`null` ist problematisch: nicht das Konzept, aber die Implementierung.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Was bringt das? `Option<T>` kann nicht als Wert verwendet werden. Er muss
entpackt werden, indem seine _Varianten_ geprüft werden.

## SP3: Pattern Matching (Matching von `Option`)

```rust
fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    return Some(a as f64 / b as f64);
}

let result = 5.5 + divide(5, 2); // error!

// correct:
let result = 5.5 + match divide(5, 2) {
    Some(c) => c,
    None => 0,
}
```

## SP3: Pattern Matching (Vergleiche/Sortierung)

```rust
let secret_number = // random value
let guess = // user input

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

## SP3: Pattern Matching (Fälle ignorieren)

Nur ein Fall von Interesse:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => (),
    Ordering::Greater => (),
    Ordering::Equal => println!("You win!"),
}
```

Vereinfachung:

```rust
match guess.cmp(&secret_number) {
    Ordering::Equal => println!("You win!"),
    _ => (), // "default" (last arm, matches everything)
}
```

## SP3: Pattern Matching (`if`/`let`)

Wenn nur ein Fall von Interesse ist:

```rust
if let Ordering::Equal = guess.cmp(&secret_number) {
    println!("You win!");
}
```

Vorteil: Weniger Code.

Nachteil: Compiler-Checks gehen verloren.

## SP4: Concurrency (Thread starten)

```rust
use std::thread;

fn main() {
    let handle: thread::JoinHandle<i32> = thread::spawn(|| {
        return 42;
    });
    println!("Wait for it...");
    match handle.join() {
        Result::Ok(v) => println!("the answer: {}", v),
        Result::Err(e) => panic!("error: {:?}", e),
    }
}
```

## SP4: Concurrency (Shared Sate: `Mutex`)

```rust
let counter = Arc::new(Mutex::new(0)); // atomic counter

thread::spawn(move || {
    {
        let mut c = counter.lock().unwrap();
        *c += 1;
    } // implicit unlock at block's end
    // do something else
});
```

## SP4: Concurrency (Message Passing: `Channel`)

```rust
let (tx, rx) = mpsc::channel();
let mut counter = 0;

// copy for (and before!) every thread
let tx_copy = mpsc::Sender::clone(&tx);

tx_copy.send(1).unwrap(); // write to channel

drop(tx_copy); // for every copy
drop(tx); // for the original

for increment in rx { // consume channel
    counter += increment;
}
```

## Technisches Team-Fazit

- einige interessante Konzepte z.B. Ownership
    - kann Probleme bereiten (z.B. Stack)
- gutes Tooling (`cargo`, `rustfmt`)
- "intelligenter" Compiler 
    - erzwingt "guten" Code
    - gibt meistens sehr gute Fehlermeldungen
- dünne Standard Library (Abhängigkeit von Libraries)
- teils gewöhnungsbedürftig (Syntax, Memory-Handling)
- Fortschritt durch Einschränkung: neues Memory-Paradigma

## Persönliches Fazit - Patrick

zwischen Rust und Go hin und her gerissen

- Vorteile von Rust (gegenüber Go):
    - ausgeklügeltes Typsystem (Generics)
    - kein Garbage Collector (Performance, Echtzeit-Anwendungen)
    - kein `null`/`nil`
    - «funktionaler»
- Vorteile von Go (gegenüber Rust):
    - mächtigere Standard Library
    - schönere, einfachere Syntax
    - _noch_ besseres Tooling
    - Google und Unix-Genies dahinter: Thompson, Pike, Kernighan (Buch)

Fazit: Ich beschäftige mich weiter mit Rust und Go ‒ und ignoriere C++.

## Persönliches Fazit - Lukas

- Ownership ist nützlich, aber gibt Probleme
- Interessante Alternative zu C
- gute Compiler-Fehlermledungen bringen sehr viel
- für kleine CLI Tool sicher sehr gut geeignet
- sehr lebendige Sprache (neue Versione, Website, ...)

Fazit: Weiterempfehlung erteilt
