pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: old_head,
        }));
        self.head = new_head;
    }

    fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();

        old_head.map(|n| {
            self.head = n.next;
            n.element
        })
        // match old_head {
        //     Some(n) => {
        //         self.head = n.next;
        //         Some(n.element)
        //     }
        //     None => None,
        // }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.element)
        // match &self.head {
        //     Some(n) => Some(&n.element),
        //     None => None,
        // }
    }

    fn len(&self) -> usize {
        let mut count = 0usize;
        let mut link = &self.head;
        while let Some(node) = link {
            count += 1;
            link = &node.next;
        }

        // while link.is_some() {
        //     count += 1;
        //     let a = link.as_ref();
        //     let b = a.unwrap();
        //     let c = &b.next;
        //     link = c;
        // }

        return count;
    }
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::<u32>::empty();
        assert_eq!(list.len(), 0);
        list.push(1024);
        list.push(42);
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), Some(1024));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }
}
