# Concurrency mit Threads

Rust versteht sich als Low-Level-Programmiersprache. Ein Thread in Rust wird
beispielsweise 1:1 auf einen Betriebssystem-Thread abgebildet. (Dies lässt sich
auf Unix-Systemen mit `top -H` nachprüfen.) In Kombination mit einer
High-Level-API lassen sich in Rust aber dennoch mit wenig Aufwand (und wenig
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
gewartet werden. Diese Methode gibt sogleich den Wert zurück, der von der
anonymen Thread-Funktion zurückgegeben wird. Dieser wird allerdings nicht
direkt, sondern in ein `Result<T>` verpackt zurückgegeben. Ein `Result<T>` ist
vergleichbar mit einer `Option<T>`, mit dem Unterschied, dass die Varianten
`Ok` und `Err` heissen, und die zweite Variante einen Fehler beinhaltet.

Das `Result` kann mittels `match` entpackt werden:

```rust
match handle.join() {
    Ok(v) => println!("The thread says: '{}'", v),
    Err(e) => panic!(e),
}
```

Der zurückgegebene Wert wird in eine zusätzliche String-Nachricht verpackt
ausgegeben. Im Fehlerfall wird das Problem mit `panic!` weiter nach oben
delegiert ‒ vergleichbar mit `throw` in Java.

Auf der Basis von Threads unterstützt Rust verschiedene Concurrency-Modelle:
Shared State und Message Passing.

## Shared State mit Mutex

Bei Shared-State-Concurrency greifen mehrere Threads wechselseitig auf
gemeinsame Speicherbereiche zu. Mittels pessimistischem Locking via Mutex wird
sichergestellt, dass sich die einzelnen Threads dabei nicht in die Quere
kommen. Möchte man etwa einen Zähler von mehreren Threads hochzählen lassen,
bietet Rust dafür einen atomaren Zähler (`Arc`: atomic reference counter).
Dieser wird mit einem `Mutex` ausgestattet, der wiederum einen Variable
schützt, die hier als Integer-Variable mit dem Wert `0` angegeben wird:

```rust
let counter = Arc::new(Mutex::new(0));
```

Ein Thread kann den Zähler nun folgendermassen erhöhen:

```rust
thread::spawn(move || {
    // do something before
    {
        let mut c = counter.lock().unwrap();
        *c += 1;
    }
    // do something else afterwards
});
```

Hier wird `move` benötigt, damit der Thread auf den Counter zugreifen kann.
Dieser bzw. dessen `Mutex` wird per `lock()` gesperrt. (Die Methode `unwrap()`
gibt den Rückgabewert von `lock()` zurück. Im Fehlerfall würde der Fehler mit
`panic` weitergereicht. Das ist kompakter als ein `match`-Konstrukt.) Nach der
Erhöhung des Zählers wird der `Mutex` _nicht_ explizit, sondern implizit am
Blockende freigegeben. Aus diesem Grund wurde hier ein zusätzlicher innerer
Block eingeschoben. Würde der Thread im gleichen Block nach der Erhöhung des
Zählers noch weitere Arbeit ausführen, bliebe die Sperre solange
aufrechterhalten.

Das Codebeispiel `mutex` zeigt eine beispielhafte Anwendung.

## Message Passing mit Channels

Eine Alternative zur fehleranfälligen Manipulation geteilter Speicherbereiche
ist das Message Passing, wobei mehrere Threads über Channels Informationen
untereinander austauschen. Das Prinzip ist mit Unix-Pipes vergleichbar, und die
Rust-Implementierung ist von Go inspiriert. (Die Go-Entwickler beziehen sich
dabei auf Tony Hoares _Communicating Sequential Processes_. Die andere bekannte
Erfindung von Tony Hoare ‒ `null` ‒ findet sich dabei auch in Go wieder, jedoch
nicht in Rust.)

Eine mögliche Channel-Implementierung in Rust ist das Modul `mpsc` (Multi
Producer, Single Consumer), das auf einer FIFO-Queue basiert. Channels haben
zwei Teile, einen Transmitter (Sender) und einen Receiver, welche man bei der
Erstellung eines Channels als Tupel erhält und idiomatisch mit `tx`
(Transmitter) und `rx` (Receiver) bezeichnet:

```rust
let (tx, rx) = mpsc::channel();
```

Jeder Thread muss seine eigene Kopie vom Transmitter anlegen, damit er auf den
Channel schreiben kann:

```rust
let tx_copy = mpsc::Sender::clone(&tx);
tx_copy.send(1).unwrap();
```

Im Hauptthread (oder einem beliebigen anderen Thread) kann der Channel über
eine Iteration konsumiert werden:

```rust
for messages in rx {
    counter += message; // getting increments from threads
}
```

Die Schleife läuft solange, bis alle Transmitter geschlossen und alle
Nachrichten auf dem Channel konsumiert sind. Dies geschieht explizit: in jedem
Thread mit `drop(tx_copy)` bzw. im Hauptthread mittels `drop(tx)`.

Das Codebeispiel `chans` implementiert das semantisch gleiche Programm wie
das Beispiel `mutex`, verwendet dazu jedoch einen Channel anstelle eines
Mutexes. Die Implementierung mit dem Channel ist dabei etwas kürzer und
eleganter.
