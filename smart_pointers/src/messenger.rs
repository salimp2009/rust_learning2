pub trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Debug)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max > 1.0 {
            self.messenger.send("Error! you are over quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used %75 of your quoata!");
        }
    }
}
#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;
    #[derive(Debug)]
    struct MessengerService {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MessengerService {
        fn new() -> Self {
            MessengerService {
                sent_messages: RefCell::new(vec![]),
            }
        }

        // add code here
    }

    impl Messenger for MessengerService {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(message.to_string());
        }
    }
}
