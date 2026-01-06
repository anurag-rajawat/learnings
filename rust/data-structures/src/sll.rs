#[derive(Debug)]
pub struct ListItem<T> {
    pub data: Box<T>,
    pub next: Option<Box<ListItem<T>>>,
}

impl<T> ListItem<T> {
    pub fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }

    pub fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next.as_ref())
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }

    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    pub head: ListItem<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }

    pub fn append(&mut self, data: T) {
        // let mut tail = self.head.mut_tail();
        // tail.next = Some(Box::new(ListItem::new(data)));
        self.head.mut_tail().next = Some(Box::new(ListItem::new(data)));
    }

    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }
}
