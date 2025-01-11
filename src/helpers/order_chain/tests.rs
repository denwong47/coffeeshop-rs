use super::*;

use std::sync::Arc;

/// Test value that implements no traits other than comparison.
#[derive(Debug, Clone, PartialEq)]
struct MyValue {
    value: i32,
}

/// Test chain type.
type MyChain = Chain<String, MyValue>;

mod instantiation {
    use super::*;

    #[tokio::test]
    async fn from() {
        let chain = MyChain::from(("hello, world!".to_string(), MyValue { value: 42 }));
        assert_eq!(chain.len().await, 1);
        let head = chain.head().await.unwrap();
        assert_eq!(head.key(), "hello, world!");
        assert_eq!(head.value(), &MyValue { value: 42 });
        assert_eq!(Arc::strong_count(&head), 2);

        drop(chain);

        assert_eq!(Arc::strong_count(&head), 1);
    }

    #[tokio::test]
    async fn new() {
        let chain = Chain::<i32, i32>::new();
        assert!(chain.head().await.is_none());
        assert_eq!(chain.len().await, 0);
    }

    #[tokio::test]
    async fn from_iter() {
        let items = vec![
            ("hello".to_string(), MyValue { value: 42 }),
            ("world".to_string(), MyValue { value: 43 }),
        ];
        let chain = Chain::from_iter(items.clone().into_iter()).await.unwrap();

        assert_eq!(chain.len().await, 2);

        let iter = chain.iter().await;
        for (actual, expected) in iter.zip(items.into_iter()) {
            assert_eq!(actual.key(), &expected.0);
            assert_eq!(actual.value(), &expected.1);
        }
    }

    #[tokio::test]
    async fn from_iter_duplicated_keys() {
        let dupe_item = ("world".to_string(), MyValue { value: 44 });
        let items = vec![
            ("hello".to_string(), MyValue { value: 42 }),
            ("world".to_string(), MyValue { value: 43 }),
            dupe_item.clone(),
        ];
        let result = Chain::from_iter(items.clone().into_iter()).await;

        assert!(
            result.is_err(),
            "Chain should not be created successfully with a duplicate key."
        );
        assert_eq!(
            result.unwrap_err(),
            AttachmentError::KeyAlreadyExists(Arc::new(dupe_item.into()))
        );
    }
}

mod insertion {
    use super::*;

    #[tokio::test]
    async fn append_on_empty() {
        let chain = Chain::<&str, MyValue>::new();

        assert_eq!(chain.len().await, 0);
        assert!(chain.head().await.is_none());

        // Insert a single item.
        chain.insert("hello", MyValue { value: 42 }).await.unwrap();

        assert_eq!(chain.len().await, 1);

        '_test_head_tail: {
            let head = chain.head().await.unwrap();
            assert_eq!(head.key(), &"hello");

            let tail = chain.tail().await.unwrap();
            assert_eq!(head, tail);

            // Insert non-duplicate key.
            chain.insert("world", MyValue { value: 43 }).await.unwrap();

            assert_eq!(chain.len().await, 2);
            assert_eq!(chain.head().await.unwrap(), head);
            assert_eq!(chain.tail().await.unwrap().key(), &"world");
        }

        // Insert duplicate key; this should error.
        chain
            .insert("hello", MyValue { value: 44 })
            .await
            .unwrap_err();

        // Create a strong reference to the second item, then advance the chain.
        let second = chain.head().await.unwrap().next().cloned();

        chain.advance().await;

        assert_eq!(
            chain.head().await,
            second,
            "The second item should now be the head of the chain."
        );

        // We should now be able to insert the duplicate key.
        chain.insert("hello", MyValue { value: 44 }).await.unwrap();

        drop(second)
    }
}

mod advance {
    use super::*;

    /// Test chain type.
    static ITEMS: [(&str, MyValue); 3] = [
        ("hello", MyValue { value: 42 }),
        ("world", MyValue { value: 43 }),
        ("goodbye", MyValue { value: 44 }),
    ];

    #[tokio::test]
    async fn no_external_references() {
        let chain = Chain::from_iter(ITEMS.iter().cloned()).await.unwrap();
        assert!(chain.head().await.is_some());

        chain.advance().await;

        let head = chain.head().await;
        assert!(head.is_none());
    }

    #[tokio::test]
    async fn head_with_external_references() {
        let chain = Chain::from_iter(ITEMS.iter().cloned()).await.unwrap();
        assert!(chain.head().await.is_some());

        let before_advancement = chain.head().await.unwrap();
        chain.advance().await;

        let after_advancement = chain.head().await;

        assert_eq!(Some(before_advancement), after_advancement);
    }

    #[tokio::test]
    async fn tail_with_external_references() {
        let chain = Chain::from_iter(ITEMS.iter().cloned())
            .await
            .expect("Chain should be created successfully.");
        assert!(chain.head().await.is_some());

        let before_advancement = chain.tail().await.unwrap();
        assert_eq!(*before_advancement.key(), "goodbye");

        chain.advance().await;

        let after_advancement = chain.head().await;

        assert_eq!(Some(before_advancement), after_advancement);
    }
}

mod thread_safety {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn insert_8192_items() {
        // Lets not fuss about the exact number of items if its not divisible by the number of workers;
        // we just want to test that the chain can handle a large number of items.
        const ASPIRED_ITEMS: usize = 4096;
        const WORKERS: usize = 4;
        const ITEMS_PER_WORKER: usize = ASPIRED_ITEMS / WORKERS;
        const ACTUAL_ITEMS: usize = WORKERS * ITEMS_PER_WORKER;

        let chain = MyChain::new();
        async fn insert_items(
            range: std::ops::Range<usize>,
            chain: &MyChain,
        ) -> Result<(), AttachmentError<Arc<ChainSegment<String, MyValue>>>> {
            let start = range.start;
            let end = range.end;
            crate::debug!("Inserting items {start} to {end} into the chain.",);
            for i in range {
                chain
                    .insert(format!("Item {i}"), MyValue { value: i as i32 })
                    .await?;
                crate::debug!("Inserted item {i} into the chain.",);
                tokio::task::yield_now().await;
            }
            crate::debug!("Inserted items {start} to {end} into the chain.",);

            Ok(())
        }

        futures::future::join_all(
            (0..ACTUAL_ITEMS)
                .step_by(ITEMS_PER_WORKER)
                .map(|range| insert_items(range..range + ITEMS_PER_WORKER, &chain)),
        )
        .await;

        // assert_eq!(chain.len().await, ACTUAL_ITEMS);
    }
}
