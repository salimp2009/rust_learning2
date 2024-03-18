use std::cell::RefCell;
use std::rc::Rc;

type ItemRef<T> = Rc<RefCell<ListItem<T>>>;

pub struct ListItem<T> {
    prev: Option<ItemRef<T>>,
    data: Box<T>,
    next: Option<ItemRef<T>>,
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
}
