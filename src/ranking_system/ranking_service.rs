use super::ranking_session::RankingSession;

pub struct RankingService<T: Clone + Eq> {
    items: Vec<T>,
}

impl<T: Clone + Eq> RankingService<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn new_session(&self) -> RankingSession<T> {
        let session = RankingSession::new(self.items.clone());

        session
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }
}
