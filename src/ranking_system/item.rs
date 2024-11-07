use std::sync::atomic::AtomicUsize;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Item<T: Clone> {
    id: usize,
    score: usize,
    value: T,
}

impl<T: Clone> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Clone> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<T: Clone + Ord> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T: Clone> Item<T> {
    pub fn new(value: T) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            score: 0,
            value,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn set_score(&mut self, score: usize) {
        self.score = score;
    }

    pub fn add_one_point(&mut self) {
        self.score += 1;
    }
}
