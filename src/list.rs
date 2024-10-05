use std::ptr::eq;
#[allow(unused_imports)]
use std::{fmt::Display, mem};

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
    // Use the implementation of this method to guide your implementation of
    // `insert` and `reverse`
    /// Deletes a node from the list
    pub fn delete(&mut self) {
        // Temporarily replaces the current node with default value (Nil).
        // In exchange, we get to take ownership of the current node instead of just
        // having it by mutable reference.
        let as_owned: ListNode<T> = mem::take(self);
        match as_owned {
            ListNode::Nil => {}
            ListNode::Cons(_, next) => {
                // Write the next node to the current node
                *self = *next;
            }
        }
    }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        ListNode::Nil
    }
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
        match self {
            ListNode::Nil => {
                *self = ListNode::Cons(value, Box::new(ListNode::Nil));
                self
            }
            ListNode::Cons(_, ref mut t) => t.insert(value),
        }
    }
    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        let mut prev = ListNode::Nil;
        let mut cur = std::mem::replace(self, ListNode::Nil);

        loop {
            match cur {
                ListNode::Nil => {
                    break;
                }
                ListNode::Cons(h, t) => {
                    let next = *t;
                    cur = prev;
                    prev = ListNode::Cons(h, Box::new(cur));
                    cur = next;
                }
            }
        }

        *self = prev;
    }
}

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        Self::Nil
    }
}

// Implement `PartialEq` for `ListNode<T>`
impl<T: PartialEq> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListNode::Nil, ListNode::Nil) => true,
            (ListNode::Cons(h1, t1), ListNode::Cons(h2, t2)) => (h1 == h2) && t1 == t2,
            _ => false,
        }
    }
}

// Implement `Eq` for `ListNode<T>`
impl<T: Eq> Eq for ListNode<T> {}

// Implement `Display` for `ListNode<T>`
impl<T: Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListNode::Nil => write!(f, "Nil"),
            ListNode::Cons(head, tail) => write!(f, "{} -> {}", head, tail),
        }
    }
}

// Implement `From<Vec<T>>` for `ListNode<T>`
impl<T> From<Vec<T>> for ListNode<T> {
    fn from(value: Vec<T>) -> Self {
        let mut res = ListNode::default();

        for val in value {
            res.insert(val);
        }

        res
    }
}

// Implement `From<ListNode<T>>` for `Vec<T>`
impl<T> From<ListNode<T>> for Vec<T> {
    fn from(value: ListNode<T>) -> Self {
        let mut res = Vec::new();
        let mut cur = value;
        loop {
            match cur {
                ListNode::Nil => {
                    break;
                }
                ListNode::Cons(h, t) => {
                    res.push(h);
                    cur = *t;
                }
            }
        }
        res
    }
}
