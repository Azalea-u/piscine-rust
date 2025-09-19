use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }
    pub fn add_element(&mut self, elem: Rc<String>) {
        self.ref_list.push(elem);
    }
    pub fn rm_all_ref(&mut self, elem: Rc<String>) {
        self.ref_list.retain(|x| Rc::ptr_eq(x, &elem));
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
