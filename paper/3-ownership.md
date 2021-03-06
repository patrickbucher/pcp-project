# Ownership
Das Ownership-Konzept in Rust ist ein wenig speziell und schwer mit anderen
Sprachen zu vergeleichen. Das Ziel des Konzeptes ist es ganz klar zu
definieren, wer für eine Variable zuständig und wie lange eine Variable gültig
ist. Durch des strenge Ownership-Konzept, welches durch den Compiler erzwungen
wird, sollte es keine Problem mit Pointern geben.

Das Ownership-Konzept von Rust wird durch die folgenden 3 Regeln definiert:

1. Jeder Wert in Rust gehört zu einer Variablen, welche ihr Owner genannt wird.
2. Es kann zur gleichen Zeit immer nur einen Owner geben.
3. Wenn der Owner den Scope verlässt, wird der Wert gelöscht.

## Heap vs. Stack
Um das Ownership-Konzept zu verstehen muss jedoch zuerst der Unterschied
zwischen Stack und Heap klar sein. Der Stack ist ein streng organisierter
Speicherbereich mit einem schnellen Zugriff. Die Daten werden gemäss
LIFO-Prinzip (last in, first out) abgelegt und es können nur Daten mit einer
fixen Grösse abgelegt werden.

Der Heap dagegen kann gebraucht werden für Daten, bei denen die Grösse zum
Kompilierungszeitpunkt noch nicht bekannt ist. Daher ist der Heap weniger stark
organisiert, da das Betriebsystem zur Laufzeit frei Speicherbereiche finden
muss. Dies führt natürlich zu längeren Zugriffszeiten.

## Skalare Datentypen
Bei skaleren Datentypen verhält sich Rust so, wie man es sich von anderen
Sprachen gewohnt ist. Zum Beispiel gibt der folgende Code den Wert 5 auf der
Konsole aus. Das liegt daran, dass diese Datentypen bei einer Zuweisung oder
einem Funktionsaufruf kopiert werden.

```rust
let x = 5;
let y = x;
println!("{}", y);
```

Die folgenden Typen implementieren das Trait `Copy` und werden daher
automatisch bei einem Aufruf oder einer Zuweisung kopiert:

- alle Integer-Datentypen, wie z.B. `u32`
- alle Fliesskommazahlen, wie z.B. `f64`
- Booleans (`bool`) und Zeichen (`char`)
- Tuples, bei denen all Datentypen das `Copy`-Trait implementieren

## Komplexere Datentypen
Bei einem sehr ähnlichen Beispiel wie dem vorherigen verhält sich der Code
jedoch nicht so, wie man es erwarten könnte, denn der Code lässt sich nicht
kompilieren. Der Compiler gibt dabei folgende Fehlermeldung zurück, welche
genau auf das oben angesprochene Problem hinweist. Der Owner eines `String`
wird bei einer Zuweisung oder einem Funktionsaufruf verändert, und daher ist der
alte Variablenname nicht mehr gültigt.

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

```
error[E0382]: use of moved value: `s1`
--> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  | value used here after move ^^
  |
  = note: move occurs because `s1` has type `std::string::String`, 
          which does not implement the `Copy` trait
```

Der Grund für dieses Verhalten ist, dass Strings zum Komplierungszeitpunkt
keine fixe Grösse haben, da diese während dem Verlauf des Programms geändert
werden können. Daher werden Strings im Heap gespeichert und nicht automatisch
bei einer Verwendung kopiert.

Damit man dennoch beispielsweise einen `String` einer Methode übergeben kann,
stellt Rust ein Konzept namens Borrowing zur Verfügung. Dabei wird einer
Funktion nicht der Wert einer Variablen übergeben, sondern eine Referenz. Auf
dieser Referenz können dann Operationen ausgeführt werden, aber der Owner der
Daten bleibt weiterhin die vorherige Variable.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", from 06");
}
```
