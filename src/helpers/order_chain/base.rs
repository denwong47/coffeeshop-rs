use std::{
    fmt::{self, Debug, Formatter},
    sync::{Arc, Weak},
};

use tokio::sync::RwLock;

use super::{AttachmentError, ChainSegment, IterChain};

#[cfg(feature = "debug")]
const LOG_TARGET: &str = "coffeeshop::helpers::order_chain";

/// A chain of elements that are ordered by their insertion order.
///
/// This is a singly linked list that is optimized for insertion and iteration; removal
/// is only supported behind the head of the chain.
///
/// For the most part, this struct aspires to behave like a [`HashMap`](std::collections::HashMap),
/// which works atomically, at the cost of lookup being `O(n)` instead of `O(1)`. For chains that
/// are expected to be small, and the [`Eq`] cost being cheap, this is a good tradeoff.
///
/// # How it works
///
/// The [`Chain`] is composed of a series of [`ChainSegment`]s, each of which can hold
/// up to one strong [`Arc`] reference to the next segment. The first segment is held
/// by the [`Chain`] itself. This asserts that each segment will always have at least
/// one strong reference to it, and nothing will be dropped until the [`Chain`] itself
/// advances its head. Any strong references that exist at that point will continue to
/// work, but the orphaned segments will be dropped as soon as they are no longer needed.
///
/// This avoids the need for the [`Chain`] to keep track of live references to each segment,
/// while still allowing atomic insertions and lookups.
///
/// The head of the chain is behind a [`RwLock`], which only needs to be write-locked when
/// the head is advanced; and the only read operations necessary is to get the head reference
/// and [`Arc::clone`] it, which is a cheap operation that should not block for extended periods
/// of time, and has the benefit of permitting read users that had begun lookup
/// operations to continue doing so while the head is being advanced.
///
/// # Warning
///
/// Due to the large amount of recursive calls when tracing down the chain,
/// this chain is best run in release mode. Debug mode will try to keep track
/// of the stack trace, which will cause the stack to overflow, which is not
/// a problem in release mode.
pub struct Chain<K, V>
where
    K: Eq,
{
    head: RwLock<Option<Arc<ChainSegment<K, V>>>>,
}

/// For any [`Chain`] that has keys and values that are [`Eq`] and [`Debug`], the [`Chain`] itself
/// can be debugged.
impl<K, V> Debug for Chain<K, V>
where
    K: Eq + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chain").field("head", &self.head).finish()
    }
}

impl<K, V> Default for Chain<K, V>
where
    K: Eq,
{
    fn default() -> Self {
        Self {
            head: RwLock::default(),
        }
    }
}

impl<K, V> From<(K, V)> for Chain<K, V>
where
    K: Eq,
{
    fn from((key, value): (K, V)) -> Self {
        Self {
            // Convert the key and value into a ChainSegment, then into an Arc, then into an Option.
            head: RwLock::new(Some(Arc::new((key, value).into()))),
        }
    }
}

impl<K, V> Chain<K, V>
where
    K: Eq + std::fmt::Debug,
{
    /// Creates a new, empty chain.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new chain from an iterator of key-value pairs.
    ///
    /// If duplicated keys are found, the chain will not be created and an error will be returned.
    /// The iterator would have been consumed up to the point of the error.
    pub async fn from_iter(
        iter: impl Iterator<Item = (K, V)>,
    ) -> Result<Self, AttachmentError<Arc<ChainSegment<K, V>>>> {
        let chain = Self::new();

        // Have to use a for loop here because the async block cannot be used in an iterator.
        for (key, value) in iter {
            chain.insert(key, value).await?;
        }

        Ok(chain)
    }

    /// Gets a new [`Arc`] strong reference to the head of the chain if it exists.
    pub async fn head(&self) -> Option<Arc<ChainSegment<K, V>>> {
        self.head.read().await.clone()
    }

    /// Gets a weak reference to the head of the chain if it exists.
    pub async fn weak_head(&self) -> Option<Weak<ChainSegment<K, V>>> {
        self.head().await.map(|arc| Arc::downgrade(&arc))
    }

    /// Gets a new [`Arc`] strong reference to the tail of the chain if it exists.
    ///
    /// This does not guarantee that by the time the reference is returned, the tail
    /// will not have been advanced. If you are using this to attach a new element
    /// to the chain, you should use the [`Chain::insert`] method instead, which
    /// will retry the operation until it succeeds.
    ///
    /// # Cost
    ///
    /// This command is `O(n)` and should be used with care.
    pub async fn tail(&self) -> Option<Arc<ChainSegment<K, V>>> {
        self.head().await.map(|head| head.tail())
    }

    /// Creates an iterator over the chain.
    pub async fn iter(&self) -> IterChain<K, V> {
        IterChain::new(self.head().await)
    }

    /// Gets the length of the chain.
    ///
    /// # Cost
    ///
    /// This command is `O(n)` and should be used with care.
    pub async fn len(&self) -> usize {
        let head_opt = self.head.read().await;

        if let Some(mut head) = head_opt.as_ref() {
            let mut count = 1;

            while let Some(next) = head.next() {
                count += 1;
                head = next;
            }

            count
        } else {
            0
        }
    }

    /// Checks if the chain is empty.
    pub async fn is_empty(&self) -> bool {
        self.head().await.is_none()
    }

    /// Get a value from the chain by its key.
    ///
    /// # Cost
    ///
    /// This command is `O(n)` and should be used with care.
    pub async fn get(&self, key: &K) -> Option<Arc<ChainSegment<K, V>>> {
        self.iter().await.find(|segment| segment.key() == key)
    }

    /// Inserts a new element into the chain.
    ///
    /// # Cost
    ///
    /// This command is `O(n)` and should be used with care.
    pub async fn insert(
        &self,
        key: K,
        value: V,
    ) -> Result<(), AttachmentError<Arc<ChainSegment<K, V>>>> {
        let segment = Arc::new((key, value).into());
        let head_opt = self.head().await;

        if let Some(head) = head_opt {
            head.attach(segment)
        } else {
            // If the chain is empty, insert the new element as the head.
            #[cfg(feature = "debug")]
            let start_time = tokio::time::Instant::now();

            crate::debug!(target: LOG_TARGET, "Inserting new head. Waiting for head to be available...");

            let mut head = self.head.write().await;

            if let Some(new_head) = head.as_ref() {
                // TOCTOU: If the head was set while we were waiting for the write lock,
                // we should try to insert the new element into the chain. This should be
                // an extremely rare occurrence, but it is possible.
                // In this
                new_head.attach(segment)
            } else {
                crate::debug!(target: LOG_TARGET, "Head is available for initiation after {:?}.", start_time.elapsed());

                *head = Some(segment);

                Ok(())
            }
        }
    }

    /// Advance the head of the chain as far as possible.
    pub async fn advance(&self) {
        #[cfg(feature = "debug")]
        let start_time = tokio::time::Instant::now();
        crate::debug!(target: LOG_TARGET, "Acquiring write access to advance head of chain...");

        // Using a write lock here prevents another thread from reading the head, which in turn
        // changes the strong count of the head, which may affect the accuracy of the next head.
        let mut head = self.head.write().await;
        crate::debug!(target: LOG_TARGET, "Write access acquired after {:?}.", start_time.elapsed());

        loop {
            if let Some(next) = head.as_ref() {
                // If the next segment has a strong count greater than 1, it
                // means that there is another reference to it, and we should
                // not advance.
                if Arc::strong_count(next) > 1 {
                    crate::debug!(target: LOG_TARGET, "Next head has strong count > 1; breaking.");
                    break;
                }

                crate::trace!(target: LOG_TARGET, "Advancing head to next segment...");
                *head = next.next().cloned();
            } else {
                // Otherwise, if the next segment is None, we have reached the end of the chain;
                // we should break.
                crate::debug!(target: LOG_TARGET, "Next head is None; breaking.");
                *head = None;
                break;
            }
        }
    }
}
