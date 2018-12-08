# Traits
Das Konzept von Traits ist sehr ähnlich zu Interfaces in anderen Sprachen. Es gibt jedoch einige Unterschiede. Ein Trait erlaubt es gleiches Verhalten für einen bestimmten Zweck zu gruppieren. Ein Trait gibt die Funktionsignaturen vor, welche für die Nutzung implementiert werden sollen. Es besteht auch die Möglichkeit, dass ein Trait für Methoden eine Standard-Implementierung zur Verfügung stellen kann. 

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

Traits entfalten ihr volles Potential erst wenn man sie benutzt um bestehenden Klasse neues Verhalten azufügen. Das heisst in Rust gibt es die Möglichkeit bestehende Klassen (eigene oder aus der Standardbibliothek) für die Verwendung im eigenen Code zu erweitern. Im folgenden Beispiel wird beispielsweise der Klasse `String` aus der Standardbibliothek eine Methode hinzugefügt, welche das erste Wort des Strings zurückgiebt. Die Implementation ist sehr minimalistisch und sollte hauptsächlich die Möglichkeit veranschaulichen.

```rust
pub trait FirstWord {
    fn first_word(&self) -> String;
}

impl FirstWord for String {
    fn first_word(&self) -> String {
        let parts: Vec<_> = self.split(" ").collect();
        parts[0].to_string()
    }
}

fn main() {
    let s = String::from("hello world");
    println!("{}", s.first_word());
}

```