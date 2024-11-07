use std::collections::VecDeque;

use super::pair::Pair;

pub trait Strategy {
    type Item: Eq;

    fn comparison_pairs(&self, items: &Vec<Self::Item>) -> VecDeque<Pair<Self::Item>>;
}
