use std::borrow::Cow;

use crate::linkedlist::ListItem;

#[derive(Debug, Clone)]
pub struct SinglyLinkedListC<'a, T>
where
    T: Clone,
{
    head: Cow<'a, ListItem<T>>,
}

impl<'a, T> SinglyLinkedListC<'a, T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        SinglyLinkedListC {
            head: Cow::Owned(ListItem::new(data)),
        }
    }

    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }

    pub fn append(&self, data: T) -> Self {
        let mut new_list = self.clone();
        let tail = new_list.head.to_mut().mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
        new_list
    }
}

