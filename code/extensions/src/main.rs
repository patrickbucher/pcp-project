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
