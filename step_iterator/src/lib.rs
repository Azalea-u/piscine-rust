use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

impl<T> StepIterator<T> 
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.current;
        
        if (self.step > current && current + self.step > self.end) ||
           (self.step < current && current + self.step < self.end) 
        {
            self.done = true;
            return Some(current);
        }

        self.current = current + self.step;
        Some(current)
    }
}
