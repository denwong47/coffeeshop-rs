use crate::CoffeeShopError;
#[allow(unused_imports)]
use std::{fs::File as StdFile, io::Write as StdWrite, path::PathBuf};
use tempfile::TempDir;
use tokio::{fs::File as TokioFile, io::AsyncReadExt};
use uuid::Uuid;

use super::serde::BUFFER_SIZE;

/// The default prefix for the buffer if not specified.
const DEFAULT_PREFIX: &str = "disk-buffer-";

/// The default extension for the buffer if not specified.
const DEFAULT_EXTENSION: &str = "bin";

/// The default logger target for the buffer.
#[cfg(feature = "debug")]
const LOG_TARGET: &str = "coffeeshop::helpers::buffer";

/// A file handler that can be either a standard file or a Tokio file.
///
/// In our case, the write handler will be a blocking file handler, and the read
/// handler will be an async file handler.
pub enum FileHandler {
    /// Unused. [`Write`](StdWrite) handlers are not stored in the buffer,
    /// they are owned by the called of [`BufferOnDisk::writer`].
    Write(StdFile),

    /// The read handler for the buffer.
    ///
    /// Upon instantiating a [`BufferOnDisk`] in [`Read`] mode, a single
    /// read handler will be created and stored in the buffer.
    Read(TokioFile),
}

/// A trait to define the state of the buffer.
pub trait BufferStateType {
    /// Get the state as a string.
    fn as_str(&self) -> &'static str;
}

/// Defines the state of the [`BufferOnDisk`].
///
/// A [`BufferOnDisk`] needs to be written to first, before it can be read from.
/// A [`Write`](Write) buffer can be transitioned to a
/// [`Read`](Read) buffer, but not the other way around.
pub struct Read {}
impl BufferStateType for Read {
    fn as_str(&self) -> &'static str {
        "read"
    }
}

/// Defines the state of the [`BufferOnDisk`].
///
/// A [`BufferOnDisk`] needs to be written to first, before it can be read from.
/// A [`Write`](Write) buffer can be transitioned to a
/// [`Read`](Read) buffer, but not the other way around.
pub struct Write {}
impl BufferStateType for Write {
    fn as_str(&self) -> &'static str {
        "write"
    }
}

/// A bytes buffer that is actually located on disk.
pub struct BufferOnDisk<'d, S: BufferStateType> {
    /// The directory to the buffer. This forces the temporary directory to be
    /// kept alive for the lifetime of the buffer.
    pub dir: &'d TempDir,

    /// The prefix for the buffer.
    pub prefix: String,

    /// The UUID of the file; this is randomly generated at the point of instantiation.
    pub uuid: Uuid,

    /// The [`FileHandler`] handle to the buffer.
    pub fhnd: Option<FileHandler>,

    _phantom: std::marker::PhantomData<S>,
}

impl<'d, S> BufferOnDisk<'d, S>
where
    S: BufferStateType,
{
    /// Get the path to the buffer.
    pub fn path(&self) -> PathBuf {
        self.dir.path().join(format!(
            "{prefix}{uuid}.{DEFAULT_EXTENSION}",
            prefix = &self.prefix,
            uuid = &self.uuid
        ))
    }

    /// Put the file handle into the buffer.
    pub fn with_file(mut self, fhnd: FileHandler) -> Self {
        self.fhnd = Some(fhnd);
        self
    }

    /// Take the file handle out of the buffer.
    pub fn take_file(&mut self) -> Option<FileHandler> {
        self.fhnd.take()
    }

    /// Touch the buffer to ensure it is writable.
    ///
    /// If the file already exists, an error will be returned.
    fn file_touch(&self) -> Result<(), CoffeeShopError> {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path())
            .inspect(
                |file| crate::trace!(target: LOG_TARGET, "Touched file at {:?} successfully.", file),
            )
            .and(Ok(()))
            .map_err(CoffeeShopError::from_io_error)
    }

    /// Get a write handle to the buffer.
    ///
    /// If the file does not exist, it will be created.
    ///
    /// # Safety
    ///
    /// This method does not guard against multiple writers being given access to the
    /// buffer. It is up to the caller to ensure that only one writer is given access
    /// to the buffer at a time.
    fn file_write(&self) -> Result<StdFile, CoffeeShopError> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.path())
            .inspect(
                |file| crate::trace!(target: LOG_TARGET, "Opened file at {:?} for writing.", file),
            )
            .map_err(CoffeeShopError::from_io_error)
    }

    /// Get the buffer as a read handle.
    async fn file_read(&self) -> Result<TokioFile, CoffeeShopError> {
        TokioFile::open(self.path())
            .await
            .map_err(CoffeeShopError::from_io_error)
    }
}

/// Allow the buffer to be dropped safely by closing the file handle.
impl<S> Drop for BufferOnDisk<'_, S>
where
    S: BufferStateType,
{
    fn drop(&mut self) {
        match self.take_file() {
            Some(FileHandler::Read(file)) => {
                drop(file);

                // If the file is in the read state already, we assume that all work
                // has been done and we can safely delete the file.
                std::fs::remove_file(self.path()).unwrap_or_else(|err| {
                    crate::debug!(
                        target: LOG_TARGET,
                        "Could not remove file at {:?}, the temporary file will remain: {:?}",
                        self.path(),
                        err
                    )
                });
            }
            Some(FileHandler::Write(file)) => {
                // If the file handle is still in the buffer, we need to close it.
                drop(file);
            }
            None => {}
        }
    }
}

impl<'d> BufferOnDisk<'d, Write> {
    /// Create a new buffer on disk.
    ///
    /// The buffer will be created in the provided directory, with the provided
    /// prefix. If no prefix is provided, the default prefix will be used.
    ///
    /// If the file was not created, or it already exists, an error will be returned.
    pub async fn new(dir: &'d TempDir, prefix: Option<&str>) -> Result<Self, CoffeeShopError> {
        let prefix = prefix.unwrap_or(DEFAULT_PREFIX);
        let uuid = Uuid::new_v4();

        let instance = Self {
            dir,
            prefix: prefix.to_string(),
            uuid,
            fhnd: None,
            _phantom: std::marker::PhantomData,
        };

        crate::debug!(target: LOG_TARGET, "Buffer created at {:?}", instance.path());

        instance.file_touch().map(|_| instance)
    }

    /// Complete the [`Write`](Write) state and transition to the
    /// [`Read`](Read) state.
    ///
    /// # Safety
    ///
    /// This method does not guard against multiple writers being given access to the
    /// buffer. It is up to the caller to ensure that only one writer is given access
    /// to the buffer at a time, and is responsible for ensuring that the buffer is
    /// flushed and closed correctly before this method is called.
    pub async fn finish(mut self) -> Result<BufferOnDisk<'d, Read>, CoffeeShopError> {
        // Swap the prefix out of the instance.
        let mut prefix = String::new();
        core::mem::swap(&mut self.prefix, &mut prefix);

        // Transition to the read state.
        let instance = BufferOnDisk {
            dir: self.dir,
            prefix,
            uuid: self.uuid,
            fhnd: None,
            _phantom: std::marker::PhantomData,
        };

        // Inject the file handle back into the instance.
        instance
            .file_read()
            .await
            .map(|file| instance.with_file(FileHandler::Read(file)))
    }

    /// Return a [`Write`](StdWrite) handler for the buffer.
    ///
    /// # Safety
    ///
    /// This method does not guard against multiple writers being given access to the
    /// buffer. It is up to the caller to ensure that only one writer is given access
    /// to the buffer at a time, and is responsible for ensuring that the buffer is
    /// closed correctly before calling [`finish`](Self::finish).
    pub fn writer(&mut self) -> Result<StdFile, CoffeeShopError> {
        self.file_write()
    }
}

impl<'d> BufferOnDisk<'d, Read> {
    /// Create a new reader for the buffer.
    pub async fn reader(&mut self) -> Result<&mut TokioFile, CoffeeShopError> {
        let path = self.path();
        if let Some(FileHandler::Read(file)) = self.fhnd.as_mut() {
            Ok(file)
        } else {
            Err(CoffeeShopError::TempFileAccessFailure {
                path,
                reason: "The file handle is available for reading.".to_string(),
            })
        }
    }

    /// Convenient method to read the whole buffer into memory.
    ///
    /// This will consume the buffer and the temporary file will be deleted.
    pub async fn read_to_end(mut self) -> Result<Vec<u8>, CoffeeShopError> {
        let reader = self.reader().await?;
        let mut output = Vec::with_capacity(BUFFER_SIZE);

        reader
            .read_to_end(&mut output)
            .await
            .and(Ok(output))
            .map_err(CoffeeShopError::from_io_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use tokio::io::AsyncReadExt;

    #[cfg(feature = "debug")]
    const LOG_TARGET: &str = "coffeeshop::helpers::buffer::tests";

    #[tokio::test]
    async fn test_buffer_on_disk() {
        const TEST_STRING: &[u8] = b"Hello, world!";

        let dir = tempdir().unwrap();
        crate::debug!(target: LOG_TARGET, "Temporary directory created at {:?}", dir.path());
        let mut buffer = BufferOnDisk::<Write>::new(&dir, None).await.unwrap();
        crate::debug!(target: LOG_TARGET, "Buffer created at {:?}", buffer.path());
        let path = buffer.path();
        assert!(path.exists());
        crate::debug!(target: LOG_TARGET, "Buffer path exists.");
        buffer
            .writer()
            .expect("Cannot create writer.")
            .write_all(TEST_STRING)
            .unwrap();

        let mut buffer = buffer.finish().await.unwrap();
        let path = buffer.path();

        {
            let reader = buffer.reader().await.expect("Cannot create reader.");
            let mut actual = Vec::new();

            reader.read_to_end(&mut actual).await.unwrap();

            crate::debug!(target: LOG_TARGET, "Read data: {:?}", String::from_utf8_lossy(&actual));

            assert_eq!(actual, TEST_STRING);
        }

        drop(buffer);
        crate::debug!(target: LOG_TARGET, "Buffer dropped. File still exists: {:?}", path.exists());

        assert!(!path.exists());
    }
}
