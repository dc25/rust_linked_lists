use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let h = self.head.take();
        let new_node = Some(Rc::new(RefCell::new(Node {
            elem,
            next: h.clone(),
            prev: None,
        })));
        self.head = new_node.clone();
        match h {
            Some(n) => n.borrow_mut().prev = new_node,
            None => self.tail = new_node,
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            match n.borrow_mut().next.take() {
                Some(next_rc) => {
                    next_rc.borrow_mut().prev = None;
                    self.head = Some(next_rc);
                }
                None => self.tail = None,
            }
            Rc::into_inner(n).unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|rch| Ref::map(rch.borrow(), |n| &n.elem))
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_mut()
            .map(|rch| RefMut::map(rch.borrow_mut(), |n| &mut n.elem))
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|rch| Ref::map(rch.borrow(), |n| &n.elem))
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut n = self.head.take();
        while let Some(rcn) = n {
            let maybe = Rc::try_unwrap(rcn).ok();
            match maybe {
                Some(definitely) => n = definitely.borrow_mut().next.take(),
                None => break,
            }
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub fn third() {
    println!("THIRD");
}

mod test {
    #[test]
    pub fn test() {
        use crate::third::List;
        let mut li = List::new();
        li.push_front(4);
        li.push_front(5);
        assert_eq!(*li.peek_front().unwrap(), 5);
        assert_eq!(*li.peek_back().unwrap(), 4);
        let pm = li.peek_front_mut();
        *pm.unwrap() = 500;
        assert_eq!(*li.peek_front().unwrap(), 500);
        assert_eq!(*li.peek_back().unwrap(), 4);

        let mut iit = li.into_iter();
        assert_eq!(iit.next(), Some(500));
        assert_eq!(iit.next(), Some(4));
        assert_eq!(iit.next(), None);

        let mut li2 = List::new();
        li2.push_front("5");
        assert_eq!(*li2.peek_front().unwrap(), "5");
        assert_eq!(*li2.peek_back().unwrap(), "5");

        assert_eq!(&*li2.peek_front().unwrap(), &"5");
        assert_eq!(&*li2.peek_back().unwrap(), &"5");
        li2.push_front("6");
        assert_eq!(*li2.peek_front().unwrap(), "6");
        assert_eq!(li2.pop_front(), Some("6"));
        assert_eq!(li2.pop_front(), Some("5"));
        assert_eq!(li2.pop_front(), None);
    }
}
