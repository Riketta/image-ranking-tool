use std::{cell::RefCell, collections::VecDeque, marker::PhantomData, rc::Rc};

use super::{
    item::Item,
    pair::Pair,
    strategies::{
        each_against_random_n_strategy::EachAgainstRandomNStrategy,
        each_with_each_strategy::EachWithEachStrategy,
    },
    strategy::Strategy,
};

pub struct NoStrategy;
pub struct WithStrategy;

pub struct RankingSession<T: Clone + Eq, State = NoStrategy> {
    items: Vec<Rc<RefCell<Item<T>>>>,
    pairs: Option<VecDeque<Pair<Rc<RefCell<Item<T>>>>>>,
    total_pairs: usize,
    state: PhantomData<State>,
}

impl<T: Clone + Eq, State> RankingSession<T, State> {
    pub fn items(&self) -> &Vec<Rc<RefCell<Item<T>>>> {
        &self.items
    }

    pub fn items_count(&self) -> usize {
        self.items.len()
    }
}

impl<T: Clone + Eq> RankingSession<T, WithStrategy> {
    pub fn total_pairs(&self) -> usize {
        self.total_pairs
    }

    pub fn current_pair_index(&self) -> usize {
        match &self.pairs {
            Some(pairs) => self.total_pairs() - pairs.len(),
            None => 0,
        }
    }
}

impl<T: Clone + Eq> RankingSession<T, NoStrategy> {
    pub fn new(items: Vec<T>) -> RankingSession<T, NoStrategy> {
        let items: Vec<Rc<RefCell<Item<T>>>> = items
            .into_iter()
            .map(|item| Rc::from(RefCell::from(Item::new(item))))
            .collect();

        Self {
            items,
            pairs: None,
            total_pairs: 0,
            state: PhantomData,
        }
    }

    pub fn with_strategy(
        self,
        strategy: impl Strategy<Item = Rc<RefCell<Item<T>>>>,
    ) -> RankingSession<T, WithStrategy> {
        let pairs = strategy.comparison_pairs(&self.items);
        let total_pairs = pairs.len();
        let pairs = Some(pairs);

        RankingSession {
            items: self.items,
            pairs,
            total_pairs,
            state: PhantomData::<WithStrategy>,
        }
    }

    pub fn with_each_with_each_strategy(self, shuffle: bool) -> RankingSession<T, WithStrategy> {
        self.with_strategy(EachWithEachStrategy::new(shuffle))
    }

    pub fn with_each_against_random_n_strategy(self, shuffle: bool, n_random: usize) -> RankingSession<T, WithStrategy> {
        self.with_strategy(EachAgainstRandomNStrategy::new(shuffle, n_random))
    }
}

impl<T: Clone + Eq> Iterator for RankingSession<T, WithStrategy> {
    type Item = Pair<Rc<RefCell<Item<T>>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pairs = self.pairs.take();

        let item = match pairs {
            Some(ref mut pairs) => pairs.pop_front(),
            None => None,
        };
        self.pairs = pairs;

        item
    }
}

#[cfg(test)]
mod tests {
    use crate::ranking_service::RankingService;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_states() -> Result<()> {
        let cars = vec!["Hyper Car", "Car", "Bicycle"];
        let expected_items_count = cars.len();
        let expected_pairs_count = 2 + 1;

        let service: RankingService<&str> = RankingService::new(cars);
        let session: RankingSession<&str, NoStrategy> = service.new_session();
        let mut session: RankingSession<&str, WithStrategy> =
            session.with_each_with_each_strategy(false);

        assert_eq!(expected_items_count, session.items_count());
        assert_eq!(expected_pairs_count, session.total_pairs());
        assert_eq!(0, session.current_pair_index());

        let pair = session.next().expect("should have value");
        {
            let mut left = pair.left.borrow_mut();
            assert_eq!(0, left.score());
            left.add_one_point();
            assert_eq!(1, left.score());
        }

        let left = &*pair.left.borrow();
        assert_eq!(&"Hyper Car", left.value());
        let right = &*pair.right.borrow();
        assert_eq!(&"Car", right.value());

        let pair = session.next().expect("should have value");
        let left = &*pair.left.borrow();
        assert_eq!(&"Hyper Car", left.value());
        let right = &*pair.right.borrow();
        assert_eq!(&"Bicycle", right.value());

        let pair = session.next().expect("should have value");
        let left = &*pair.left.borrow();
        assert_eq!(&"Car", left.value());
        let right = &*pair.right.borrow();
        assert_eq!(&"Bicycle", right.value());

        let pair = session.next();
        assert!(pair.is_none());

        Ok(())
    }
}
