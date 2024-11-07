use std::{cell::RefCell, hash::Hash, rc::Rc};

use super::item::Item;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Pair<T: Eq> {
    pub left: T,
    pub right: T,
}

impl<T: PartialEq + Eq> PartialEq for Pair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
            || self.left == other.right && self.right == other.left
    }
}

impl<T: Clone + Eq> Pair<T> {
    pub fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T: Clone + Eq> Hash for Pair<Rc<RefCell<Item<T>>>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let left_id = self.left.borrow().id();
        let right_id = self.left.borrow().id();

        if left_id >= right_id {
            left_id.hash(state);
            right_id.hash(state);
        } else {
            right_id.hash(state);
            left_id.hash(state);
        }
    }
}
