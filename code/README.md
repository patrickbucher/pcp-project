# Ausführung

Die meisten Codebeispiele verwenden die `cargo`-Projektstruktur. Sie verfügen
über eine Datei namens `Cargo.toml`, und der Code liegt im Unterverzeichnis
`src`. Ein solches Programm lässt sich beispielsweise folgendermassen
ausführen:

    $ cd guessing_game
    $ cargo run

Bei anderen Codebeispielen handelt es sich nicht um `cargo`-Projekte. Diese
müssen manuell kompiliert und ausgeführt werden:

    $ cd ownership
    $ rustc 01-scalar-types.rs
    $ ./scalar-types

# Übungen

- Woche 1, Aufgabe 3: einfacher Stack
    - `stack`
- Woche 9, Aufgabe 2: Zufallszahlen
    - `random`
- Woche 9, Aufgabe 3: nebenläufige Tasks
    - `concurrency`

# Weitere Code-Beispiele

1. Ownership-Konzept: zahlreiche Codebeispiele
    - `ownership`
2. Traits: 
    - `traits`: Summary-Beispiel
    - `extensions`: Erweiterung der String-Klasse
3. Pattern Matching: Pythagorean Triplets
    - `pattern-matching`
4. Concurrency: Zählerbeispiel
    - `mutex`: Mutex-Implementierung
    - `channel`: Channel-Implementierung

Der Ordner `guessing_game` enthält ein Ratespiel, das im [2. Kapitel des
Rust-Buches](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
erstellt wird.
