use std::sync::{Arc, OnceLock};

const LOG_TARGET: &str = "coffeeshop::helpers::order_chain::segment";

use super::errors::AttachmentError;

/// A segment of the linked list.
#[derive(Debug)]
pub struct ChainSegment<K, V> {
    key: K,
    inner: V,
    next: OnceLock<Arc<Self>>,
}

impl<K, V> PartialEq for ChainSegment<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.inner == other.inner
    }
}

impl<K, V> From<(K, V)> for ChainSegment<K, V> {
    /// Creates a new segment from a key-value pair.
    fn from((key, inner): (K, V)) -> Self {
        Self {
            key,
            inner,
            next: OnceLock::new(),
        }
    }
}

impl<K, V> ChainSegment<K, V> {
    /// Gets the key of the segment.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Gets the inner value of the segment.
    pub fn value(&self) -> &V {
        &self.inner
    }

    /// Gets the next segment in the chain if it exists.
    pub fn next(&self) -> Option<&Arc<Self>> {
        self.next.get()
    }

    /// From this segment, transverse the chain until the end.
    pub fn tail(self: &Arc<Self>) -> Arc<Self> {
        let mut current: &Arc<ChainSegment<K, V>> = self;
        while let Some(next) = current.next() {
            current = next;
        }
        current.clone()
    }
}

impl<K, V> ChainSegment<K, V>
where
    K: PartialEq + std::fmt::Debug,
{
    /// Attach a new segment to the chain.
    ///
    /// If it was able to attach the new segment, it will return [`Ok`].
    /// Otherwise, it will return the input segment unattached wrapped in the
    /// [`AttachmentError`] variant that corresponds to the error.
    pub fn try_attach(
        self: &Arc<Self>,
        next: Arc<Self>,
    ) -> Result<Arc<Self>, AttachmentError<Arc<Self>>> {
        if self.key == next.key {
            Err(AttachmentError::KeyAlreadyExists {
                existing: Arc::clone(self),
                candidate: next,
            })
        } else {
            self.next.set(next).map_err(AttachmentError::NotTail)?;

            Ok(Arc::clone(self.next().unwrap()))
        }
    }

    /// Attach a new segment to the chain.
    ///
    /// This will iterate through the chain until it finds the last segment;
    /// then it will attach the new segment to the last segment.
    ///
    /// This will succeed if the key of the new segment does not exist in the chain.
    /// This cannot return a [`AttachmentError::NotTail`] error. If the chain is modified
    /// in between the time the tail is found and the time the new segment is attached,
    /// this will iterate through the chain again to find the new tail.
    ///
    /// This operation is thread-safe.
    ///
    /// This does not return the reference to the new segment; if you need it,
    /// clone the reference to the new segment before attaching it.
    ///
    /// # Cost
    ///
    /// This is an `O(n)` operation.
    pub fn attach(
        self: &Arc<Self>,
        next: Arc<Self>,
    ) -> Result<Arc<Self>, AttachmentError<Arc<Self>>> {
        let mut tail = Arc::clone(self);
        let mut result = tail.try_attach(next);

        // If another thread added a segment in between the time we found the tail
        // and the time we tried to attach the new segment, we will iterate through
        // the chain again to find the new tail.
        while let Err(AttachmentError::NotTail(next)) = result {
            tail = tail.tail();
            result = tail.try_attach(next);
        }

        result
    }
}

#[cfg(feature = "debug")]
impl<K, V> Drop for ChainSegment<K, V> {
    fn drop(&mut self) {
        crate::trace!(
            target: LOG_TARGET,
            "A segment has no more references; dropping."
        )
    }
}
