use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    marker::PhantomData,
    rc::Rc,
};

use rand::{seq::SliceRandom, Rng};

use crate::{item::Item, pair::Pair, ranking_system::strategy::Strategy};

pub struct EachAgainstRandomNStrategy<T> {
    shuffle: bool,
    n_random: usize,
    value: PhantomData<T>,
}

impl<T> EachAgainstRandomNStrategy<T> {
    pub fn new(shuffle: bool, n_random: usize) -> Self {
        Self {
            shuffle,
            n_random,
            value: PhantomData,
        }
    }
}

impl<T: Clone + Eq> Strategy for EachAgainstRandomNStrategy<T> {
    type Item = Rc<RefCell<Item<T>>>;
    fn comparison_pairs(&self, items: &Vec<Self::Item>) -> VecDeque<Pair<Self::Item>> {
        let pairs = items.len() * self.n_random;
        let mut queue: HashSet<Pair<Self::Item>> = HashSet::with_capacity(pairs);
        let mut rng = rand::thread_rng();

        for left_index in 0..items.len() {
            let target_pairs = queue.len() + self.n_random;

            while queue.len() < target_pairs {
                let right_index = rng.gen_range(0..items.len());

                if right_index == left_index {
                    continue;
                }

                let left = Rc::clone(&items[left_index]);
                let right = Rc::clone(&items[right_index]);

                let pair = Pair { left, right };
                queue.insert(pair);
            }
        }

        let mut queue: VecDeque<Pair<Rc<RefCell<Item<T>>>>> = queue.into_iter().collect();

        if self.shuffle {
            queue.make_contiguous().shuffle(&mut rng);
        }

        assert!(queue.len() == pairs, "Pairs: {} != {}.", queue.len(), pairs);

        queue
    }
}
