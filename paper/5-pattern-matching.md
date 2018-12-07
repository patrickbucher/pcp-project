# Pattern Matching

Viele Programmiersprachen verwenden `null`, um die Abwesenheit eines Wertes zu
signalisieren. Das Konzept ist an sich sinnvoll, nur erlauben es die meisten
Programmiersprachen, dass `null` (oder `nil`, oder `None`) wie ein normaler
Wert verwendet werden kann. Dies führt zu schwerwiegenden Laufzeitfehlern.

## `Option<T>` statt `null`

Rust kennt kein `null`. Stattdessen wird die An- und Abwesenheit eines Wertes
mit der Enumeration `Option<T>` gelöst, welche mit Javas `Optional`
vergleichbar und folgendermassen definiert ist:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Die Enumeration `Option` hat einen Typparameter `T`, ist also generisch. Die
_Variante_ `Some` hält einen Wert des entsprechenden Typs `T`. Die Variante
`None` hat keinen Wert. Auf den ersten Blick sieht es so aus, dass das Problem
mit `null` nur verschoben und mit `None` anders bezeichnet wird. Der
Rust-Compiler stellt jedoch sicher, dass ein Ausdruck vom Typ `Option<T>` nicht
einfach so wie ein Ausdruck vom Typ `T` verwendet werden kann. Betrachten wir
folgende Funktion, die zwei Ganzzahlen dividiert und das Ergebnis als
`Option` einer Gleitkommazahl zurückgibt:

```rust
fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    return Some(a as f64 / b as f64);
}
```

Wird die Rückgabe wie ein normaler Wert behandelt, kann das Programm nicht
kompiliert werden:

```rust
let x = 10 + divide(10, 3);
```

    $ cargo check
    let x = 10 + divide(10, 3);
               ^ no implementation for `{integer} + std::option::Option<f64>`

Der Ausdruck vom Typ `Option<f64>` muss zuerst entpackt werden, um an den Wert
vom Typ `f64` heranzukommen. Hier kommt das _Pattern Matching_ ins Spiel, womit
ein `enum`-Ausdruck der jeweiligen Variante zugeordnet wird:

```rust
match divide(10, 3) {
    Some(x) => println!("{}", x),
    None => println!("fail"),
}
```

## Erzwungene Handhabung aller Fälle

`match` ist vergleichbar mit `switch` in Java. Im Kontext mit `enum`-Ausdrücken
stellt der Rust-Compiler jedoch sicher, dass jede Variante in einem eigenen
_Arm_ behandelt wird. Würde im obigen Beispiel der zweite Arm weggelassen,
scheiterte die Kompilierung:

    match divide(10, 3) {
          ^^^^^^^^^^^^^ pattern `None` not covered

Mittels Pattern Matching eliminiert Rust eine weitere Fehlerklasse: das
unvollständige Abdecken von Fällen.

Ein weiteres Anwendungsbeispiel von `match` ist der Vergleich von Zahlen bzw.
deren Sortierung. In Java gibt ein Ausdruck `a.compareTo(b)` bei `a>b` eine
positive Zahl, bei `a<b` eine negative Zahl und bei `a==b` die Zahl null
zurück. Bei Rust retournieren Vergleiche eine `enum` vom Typ `Ordering`. Im
folgenden Beispiel wird eine geratene Zahl `guess` mit einer zuvor festgelegten
Zahl `secret_number` verglichen:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("Right guess!"),
}
```

So wird wiederum sichergestellt, dass auf alle möglichen Ergebnisse reagiert
wird. Das Weglassen eines Armes würde wiederum zu einem Kompilierfehler führen.

Gibt es Fälle, auf die man im jeweiligen Kontext nicht reagieren möchte, kann
man diese einfach dem _Einheitswert_ () zuordnen:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => (),
    Ordering::Greater => (),
    Ordering::Equal => println!("Right guess!"),
}
```

Da diese Syntax etwas umständlich ist, können Fälle, die nicht von Interesse
sind, mit den Platzhalter `_` zusammengefasst werden. Da dieser auf alle Werte
passt, muss er als letzter Arm aufgeführt sein:

```rust
match guess.cmp(&secret_number) {
    Ordering::Equal => println!("Right guess!"),
    _ => (),
}
```

## Vereinfachung mit `if`/`let`

Da bei vielen Operationen nur auf eine einzige Art von Ergebnis reagiert werden
soll, bietet Rust eine kürzeres Konstrukt für solche Fälle an. Mit `if`/`let`
kann der vorherige `match`-Ausdruck folgendermassen umgeschrieben werden:

```rust
if let Ordering::Equal = guess.cmp(&secret_number) {
    println!("Right guess!");
}
```

Der Nachteil an diese Konstrukt ist, dass der Compiler nicht prüft, ob alle
möglichen Fälle abgedeckt werden.
