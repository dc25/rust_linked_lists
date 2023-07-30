use std::rc::Rc;

type Link<T>=Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
}


impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn prepend(&self, elem: T) -> Self {
        let new_node = Some(Rc::new(Node{elem, next: self.head.clone()}));
        List{head: new_node}
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }

    fn tail(&self) -> List<T> {
        List{ head: self.head.as_ref().and_then(|n| n.next.clone()) }
    }

    fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }
}

struct Iter<'a, T> (Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item=&'a T;
    fn next(&mut self) ->  Option<Self::Item> {
        self.0.map(|n| {
            self.0 = n.next.as_deref();
            &n.elem
        })

    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut h = self.head.take();
        while let Some(rcn) = h {
            match Rc::try_unwrap(rcn).ok() {
                Some(mut n) => {h = n.next.take(); }
                None  => { break }
            }
        }
    }
}

pub fn second() {
    let ll0 = List::new();
    assert_eq!(ll0.peek(), None);
    let ll1 = ll0.prepend(1);
    assert_eq!(ll1.peek(), Some(&1));
    ll1.prepend(2);
    let ll2 = ll1.prepend(2);
    assert_eq!(ll2.peek(), Some(&2));
    assert_eq!(ll2.tail().peek(), Some(&1));
    assert_eq!(ll2.tail().tail().peek(), None);

    let mut it = ll2.iter();
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), None);
}

mod test {
    #[test]
    fn test() {
        use crate::second::List;
        let ll0 = List::new();
        assert_eq!(ll0.peek(), None);
        let ll1 = ll0.prepend(1);
        assert_eq!(ll1.peek(), Some(&1));
        ll1.prepend(2);
        let ll2 = ll1.prepend(2);
        assert_eq!(ll2.peek(), Some(&2));
        assert_eq!(ll2.tail().peek(), Some(&1));
        assert_eq!(ll2.tail().tail().peek(), None);
        
        let mut it = ll2.iter();
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);

    }
}
