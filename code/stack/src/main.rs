use std::mem;

pub struct Stack {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Element>),
}

struct Element {
    value: i32,
    next: Link,
}

impl Stack {
    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_element = Box::new(Element {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_element);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(element) => {
                let element = *element;
                self.head = element.next;
                Some(element.value)
            }
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

fn main() {
    println!("Please run tests!")
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}