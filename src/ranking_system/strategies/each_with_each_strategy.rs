use std::{cell::RefCell, collections::VecDeque, marker::PhantomData, rc::Rc};

use rand::seq::SliceRandom;

use crate::{item::Item, pair::Pair, ranking_system::strategy::Strategy};

pub struct EachWithEachStrategy<T> {
    shuffle: bool,
    value: PhantomData<T>,
}

impl<T> EachWithEachStrategy<T> {
    pub fn new(shuffle: bool) -> Self {
        Self {
            shuffle,
            value: PhantomData,
        }
    }
}

impl<T: Clone + Eq> Strategy for EachWithEachStrategy<T> {
    type Item = Rc<RefCell<Item<T>>>;
    fn comparison_pairs(&self, items: &Vec<Self::Item>) -> VecDeque<Pair<Self::Item>> {
        let pairs = (0..items.len()).sum();
        let mut queue: VecDeque<Pair<Rc<RefCell<Item<T>>>>> = VecDeque::with_capacity(pairs);

        for i in 0..items.len() {
            for j in i + 1..items.len() {
                let left = Rc::clone(&items[i]);
                let right = Rc::clone(&items[j]);
                queue.push_back(Pair::new(left, right));
            }
        }

        if self.shuffle {
            let mut rng = rand::thread_rng();
            queue.make_contiguous().shuffle(&mut rng);
        }

        queue
    }
}
