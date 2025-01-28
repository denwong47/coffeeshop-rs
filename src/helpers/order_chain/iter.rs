use std::sync::Arc;

use super::ChainSegment;

/// An iterator over the key-value pairs of a chain.
///
/// This is typically created
pub struct IterChain<K, V>
where
    K: Eq,
{
    pointer: Option<Arc<ChainSegment<K, V>>>,
}

impl<K, V> Iterator for IterChain<K, V>
where
    K: Eq,
{
    type Item = Arc<ChainSegment<K, V>>;

    fn next(&mut self) -> Option<Self::Item> {
        // This prevents the subsequent code to swap the pointer with another None.
        self.pointer.as_ref()?;

        let mut next = self
            .pointer
            .as_ref()
            .and_then(|segment| segment.next().cloned());
        std::mem::swap(&mut next, &mut self.pointer);

        next
    }
}

impl<K, V> IterChain<K, V>
where
    K: Eq,
{
    /// Creates a new iterator over the chain.
    ///
    /// If a [`None`] is passed, the iterator will be empty.
    pub fn new(pointer: Option<Arc<ChainSegment<K, V>>>) -> Self {
        Self { pointer }
    }
}
