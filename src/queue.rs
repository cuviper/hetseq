
/// Heterogenous queue
/// Supports pushing, splitting to init and last
/// Mapping and folding
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Queue<Q>(pub Q);

impl Queue<()> {
    pub fn new() -> Queue<()> {
        Queue(())
    }
}

impl<Q> Queue<Q> {
    pub fn push<V>(self, value: V) -> Queue<(Queue<Q>, V)> {
        Queue((self, value))
    }
}

impl<H, T> Queue<(Queue<H>, T)> {
    pub fn init(&self) -> &Queue<H> {
        &(self.0).0
    }
    pub fn last(&self) -> &T {
        &(self.0).1
    }
}