# Traits

## What is a trait?
- is similar to interfaces in other languages
    - although with some differences
- allows to group method signatures together 
- defines a set of behaviors for a given purpose

## Define a traits
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implement a trait
```rust
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

## Use a trait
```rust
let tweet = Tweet {
    username: String::from("@demo"),
    // ..
};

println!("{}", tweet.summarize());
```

## Default implementation of methods
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## Use for class extension
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