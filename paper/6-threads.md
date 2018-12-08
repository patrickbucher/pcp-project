# Concurrency mit Threads

Rust versteht sich als Low-Level-Programmiersprache. Ein Thread in Rust wird
somit 1:1 auf einen Betriebssystem-Thread abgebildet. (Dies lässt sich auf
Unix-Systemen mit `top -H` nachprüfen.) In Kombination mit einer High-Level-API
lassen sich in Rust aber dennoch mit wenig Aufwand (und wenig
Code) elegent Probleme nebenläufig lösen.

Ein Thread wird über die assoziierte Funktion (vgl. statische Methode) `spawn`
gestartet, indem als Parameter eine anonyme Funktion mitgegeben wird. Zurück
erhält man einen `JoinHandle<T>`, wobei die Typangabe dem Rückgabetyp der
übergebenen anonymen Funktion entspricht:

```rust
let handle: thread::JoinHandle<String> = thread::spawn(|| {
    String::from("I'm a thread.")
});
```

Die beiden Pipes (`||`) nach `spawn` bezeichnen eine leere Parameterliste für
die anonyme Funktion, die dem Thread mitgegeben wird. Würde die Thread-Funktion
Werte vom umschliessenden Gültigkeitsbereich verwenden, müsste deren Ownership
zunächst mit `move` übergeben werden:

```rust
thread::spawn(move || {
    // use values from surrounding scope
});
```

Auf die Abarbeitung vom Thread kann mit der Methode `join()` von `handle`
gewartet werden. Diese Methode gibt auch sogleich den Wert zurück, der von der
anonymen Thread-Funktion zurückgegeben wird. Dieser wird allerdings nicht
direkt, sondern in ein `Result<T>` verpackt zurückgegeben. Ein `Result<T>` ist
vergleichbar mit einer `Option<T>`, mit dem Unterschied, dass die Varianten
`Ok` und `Err` heissen, und die zweite Variante einen Fehler beinhaltet. Das
`Result` kann mittels `match` entpackt werden:

```rust
match handle.join() {
    Ok(v) => println!("The thread says: '{}'", v),
    Err(e) => panic!(e),
}
```

Der zurückgegebene Wert wird in eine zusätzliche String-Nachricht verpackt
ausgegeben. Im Fehlerfall wird das Problem mit `panic!` weiter nach oben
delegiert ‒ vergleichbar mit `throw` in Java.
