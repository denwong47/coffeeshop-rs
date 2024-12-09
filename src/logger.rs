//! Centralised logging for the coffee shop.
//!

#[cfg(feature = "debug")]
use std::sync::OnceLock;

/// Lock for initialising the logger.
#[cfg(feature = "debug")]
static INIT: OnceLock<()> = OnceLock::new();

/// Initialises the logger.
pub fn init() {
    #[cfg(feature = "debug")]
    INIT.get_or_init(env_logger::init);
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::trace!(target: $target, $($arg)+);
        }
    );

    ($($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::trace!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::debug!(target: $target, $($arg)+);
        }
    );

    ($($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::debug!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::info!(target: $target, $($arg)+);
        }
    );

    ($($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::info!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::warn!(target: $target, $($arg)+);
        }
    );

    ($($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::warn!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::error!(target: $target, $($arg)+);
        }
    );

    ($($arg:tt)+) => (
        {
            $crate::logger::init();
            #[cfg(feature = "debug")]
            log::error!($($arg)+);
        }
    )
}
