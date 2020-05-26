use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> SimpleLinkedList<T> {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            node = &n.next;
            count += 1;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let mut node = Node {
            data: _element,
            next: None,
        };
        if let Some(n) = self.head.take() {
            node.next = Some(n)
        }
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(t) => {
                self.head = t.next;
                Some(t.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(t) => Some(&t.data),
            None => None,
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut s = SimpleLinkedList::new();
        let mut node = self.pop();
        while let Some(t) = node {
            s.push(t);
            node = self.pop()
        }
        s
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        for i in iter {
            ll.push(i)
        }
        ll
    }
}
// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut s = self.rev();
        let mut node = s.pop();
        while let Some(t) = node {
            v.push(t);
            node = s.pop()
        }
        v
    }
}
