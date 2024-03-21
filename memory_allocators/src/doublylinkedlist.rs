use std::cell::RefCell;
use std::clone;
use std::rc::Rc;

type ItemRef<T> = Rc<RefCell<ListItem<T>>>;

#[derive(Clone)]
pub struct ListItem<T> {
    pub prev: Option<ItemRef<T>>,
    data: Box<T>,
    pub next: Option<ItemRef<T>>,
}

pub struct DoublyLinkedList<T> {
    head: ItemRef<T>,
}

impl<T> ListItem<T> {
    pub fn new(data: T) -> Self {
        ListItem {
            prev: None,
            data: Box::new(data),
            next: None,
        }
    }

    pub fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new(data: T) -> Self {
        DoublyLinkedList {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }
    pub fn head(&self) -> ItemRef<T> {
        self.head.clone()
    }

    pub fn tail(&self) -> ItemRef<T> {
        todo!()
    }
    fn find_tail(item: ItemRef<T>) -> ItemRef<T> {
        if let Some(next) = &item.borrow().next {
            Self::find_tail(next.clone())
        } else {
            item.clone()
        }
    }

    pub fn append(&mut self, data: T) {
        let tail = Self::find_tail(self.head.clone());
        let new_item = Rc::new(RefCell::new(ListItem::new(data)));
        new_item.borrow_mut().prev = Some(tail.clone());
        tail.borrow_mut().next = Some(new_item);
    }
}
