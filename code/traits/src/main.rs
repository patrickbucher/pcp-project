pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("NEWS {}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("TWEET {}: {}", self.username, self.content)
    }
}

pub struct Feed {
    pub items: Vec<Box<Summary>>,
}

impl Feed {
    pub fn print(&self) {
        for item in self.items.iter() {
            println!("{}", item.summarize());
        }
    }

    pub fn add<T: Summary + 'static> (&mut self, item: T) {
        self.items.push(Box::new(item));
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@demo"),
        content: String::from("of course!"),
    };
    
    let article = NewsArticle {
        headline: String::from("New subject this year!"),
        author: String::from("HSLU"),
        content: String::from("Lorem ipsum dolor sit ame"),
    };

    let mut feed = Feed { items: Vec::new() };
    feed.add(tweet);
    feed.add(article);
    feed.print();
}
