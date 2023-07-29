type Link<T> = Option<Box<Node<T>>>;

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

    fn push(&mut self, elem: T) {
        let head = self.head.take();
        let new_head = Some(Box::new(Node { elem, next: head }));
        self.head = new_head;
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(mut n) = next {
            next = n.next.take();
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|n| {
            self.0.head = n.next;
            n.elem
        })
    }
}

impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|n| {
            self.0 = n.next.as_deref();
            &n.elem
        })
    }
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }
}

struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|n| {
            self.0 = n.next.as_deref_mut();
            &mut n.elem
        })
    }
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }
}

pub fn first() {
    let mut ll1 = List::new();
    assert_eq!(ll1.peek(), None);
    ll1.push(1);
    assert_eq!(ll1.peek(), Some(&1));
    ll1.push(2);
    assert_eq!(ll1.peek(), Some(&2));
    assert_eq!(ll1.pop(), Some(2));
    assert_eq!(ll1.peek(), Some(&1));
    let llpm = ll1.peek_mut();
    assert_eq!(llpm, Some(&mut 1));
    if let Some(i) = llpm {
        *i = 100;
    }
    assert_eq!(ll1.peek(), Some(&100));
    ll1.push(1000);

    {
        let mut ii1 = ll1.iter_mut();
        assert_eq!(ii1.next(), Some(&mut 1000));
        let iipm = ii1.next();
        assert_eq!(iipm, Some(&mut 100));
        if let Some(i) = iipm {
            *i = 200;
        }
        assert_eq!(ii1.next(), None);
    }

    let mut ii1 = ll1.iter();
    assert_eq!(ii1.next(), Some(&1000));
    assert_eq!(ii1.next(), Some(&200));
    assert_eq!(ii1.next(), None);

    let mut ii2 = ll1.into_iter();
    assert_eq!(ii2.next(), Some(1000));
    assert_eq!(ii2.next(), Some(200));
    assert_eq!(ii2.next(), None);
}

mod test {
    #[test]
    fn test() {
        use crate::first::List;
        let mut ll1 = List::new();
        assert_eq!(ll1.peek(), None);
        ll1.push(1);
        assert_eq!(ll1.peek(), Some(&1));
        ll1.push(2);
        assert_eq!(ll1.peek(), Some(&2));
        assert_eq!(ll1.pop(), Some(2));
        assert_eq!(ll1.peek(), Some(&1));
        let llpm = ll1.peek_mut();
        assert_eq!(llpm, Some(&mut 1));
        llpm.map(|i| *i = 100);
        assert_eq!(ll1.peek(), Some(&100));
        ll1.push(1000);

        let mut ii1 = ll1.into_iter();
        assert_eq!(ii1.next(), Some(1000));
        assert_eq!(ii1.next(), Some(100));
        assert_eq!(ii1.next(), None);
    }

    #[test]
    fn test2() {
        use crate::first::List;
        let mut ll1 = List::new();
        assert_eq!(ll1.peek(), None);
        ll1.push(1);
        assert_eq!(ll1.peek(), Some(&1));
        ll1.push(2);
        assert_eq!(ll1.peek(), Some(&2));
        assert_eq!(ll1.pop(), Some(2));
        assert_eq!(ll1.peek(), Some(&1));
        let llpm = ll1.peek_mut();
        assert_eq!(llpm, Some(&mut 1));
        llpm.map(|i| *i = 100);
        assert_eq!(ll1.peek(), Some(&100));
        ll1.push(1000);

        let mut ii1 = ll1.iter();
        assert_eq!(ii1.next(), Some(&1000));
        assert_eq!(ii1.next(), Some(&100));
        assert_eq!(ii1.next(), None);
    }

    #[test]
    fn test3() {
        use crate::first::List;
        let mut ll1 = List::new();
        assert_eq!(ll1.peek(), None);
        ll1.push(1);
        assert_eq!(ll1.peek(), Some(&1));
        ll1.push(2);
        assert_eq!(ll1.peek(), Some(&2));
        assert_eq!(ll1.pop(), Some(2));
        assert_eq!(ll1.peek(), Some(&1));
        let llpm = ll1.peek_mut();
        assert_eq!(llpm, Some(&mut 1));
        llpm.map(|i| *i = 100);
        assert_eq!(ll1.peek(), Some(&100));
        ll1.push(1000);

        {
            let mut ii1 = ll1.iter_mut();
            assert_eq!(ii1.next(), Some(&mut 1000));
            let iipm = ii1.next();
            assert_eq!(iipm, Some(&mut 100));
            iipm.map(|i| *i = 200);
            assert_eq!(ii1.next(), None);
        }

        let mut ii1 = ll1.iter();
        assert_eq!(ii1.next(), Some(&1000));
        assert_eq!(ii1.next(), Some(&200));
        assert_eq!(ii1.next(), None);
    }
}
