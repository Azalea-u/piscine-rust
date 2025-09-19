use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, cmd: String) -> (usize, Thread) {
        let mut states = self.states.borrow_mut();
        states.push(false);
        let pid = states.len() - 1;
        (pid, Thread::new(pid, cmd, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, pid: usize) -> bool {
        self.states.borrow()[pid]
    }

    pub fn drop_thread(&self, pid: usize) {
        let mut states = self.states.borrow_mut();
        if pid >= states.len() {
            panic!("Thread {} does not exist", pid);
        }
        if states[pid] {
            panic!("{} is already dropped", pid);
        }
        states[pid] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(pid: usize, cmd: String, parent: &'a ThreadPool) -> Self {
        Thread { pid, cmd, parent }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}
