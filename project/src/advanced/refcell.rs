pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max > 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max > 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

pub fn test_refcell() {
  // let x = 5;
  // let y = &mut x; // error because trying mutable reference of immutable variable
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  // A mock object that can modify itself to keep track of the messages it has seen.
  struct MockMessenger {
    send_messages: RefCell<Vec<String>>, // use immutable reference as mutable
  }
  impl MockMessenger {
    // an associated function
    fn new() -> MockMessenger {
      MockMessenger {
        send_messages: RefCell::new(vec![]),
      }
    }
  }
  impl Messenger for MockMessenger {
    // send still has an immutable borrow of self
    fn send(&self, message: &str) {
      self.send_messages.borrow_mut().push(String::from(message));
      // ・borrow_mut returns smart pointer type RefMut<T> (borrow returns sp type Ref<T>)
      //
      // let mut one_borrow = self.send_messages.borrow_mut();
      // let mut two_borrow = self.send_messages.borrow_mut();
      // ・Panicked at 'already borrowed: BorrowMutError' at second borrowing
      // ・This makes two mutable references in the same scope, which isn’t allowed
      // one_borrow.push(String::from(message));
      // two_borrow.push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.send_messages.borrow().len(), 2);
  }
}
