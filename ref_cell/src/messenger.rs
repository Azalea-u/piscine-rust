use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value<T>(&self, rc: &Rc<T>) {
        let count = Rc::strong_count(rc);
        let usage_percent = (count as f64 / self.max as f64) * 100.0;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else if usage_percent > 70.0 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                usage_percent as usize
            ));
            *self.value.borrow_mut() = count;
        } else {
            *self.value.borrow_mut() = count;
        }
    }

    pub fn peek<T>(&self, rc: &Rc<T>) {
        let count = Rc::strong_count(rc);
        let usage_percent = (count as f64 / self.max as f64) * 100.0;

        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            usage_percent as usize
        ));
    }
}
