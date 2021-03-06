use super::internal::*;

pub struct NoopConsumer;

impl NoopConsumer {
    pub fn new() -> Self {
        NoopConsumer
    }
}

impl<T> Consumer<T> for NoopConsumer {
    type Folder = NoopConsumer;
    type Reducer = NoopReducer;
    type Result = ();

    fn cost(&mut self, cost: f64) -> f64 {
        cost
    }

    fn split_at(self, _index: usize) -> (Self, Self, NoopReducer) {
        (NoopConsumer, NoopConsumer, NoopReducer)
    }

    fn into_folder(self) -> Self {
        self
    }
}

impl<T> Folder<T> for NoopConsumer {
    type Result = ();

    fn consume(self, _item: T) -> Self {
        self
    }

    fn complete(self) {}
}

impl<T> UnindexedConsumer<T> for NoopConsumer {
    fn split_off_left(&self) -> Self {
        NoopConsumer
    }

    fn to_reducer(&self) -> NoopReducer {
        NoopReducer
    }
}

pub struct NoopReducer;

impl Reducer<()> for NoopReducer {
    fn reduce(self, _left: (), _right: ()) {}
}

