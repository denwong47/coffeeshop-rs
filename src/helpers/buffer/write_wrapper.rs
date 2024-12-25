use std::io::Write as StdWrite;

/// A wrapper that takes an exclusive, mutable reference to a writer and
/// allows for writing to it. This can be passed as an owned value to
/// functions that require an owned writer, while writing to the underlying
/// writer that can still be accessed after this wrapper is dropped.
pub struct WriteWrapper<'w, T: StdWrite> {
    writer: &'w mut T,
}

impl<'w, T: StdWrite> WriteWrapper<'w, T> {
    /// Create a new WriteWrapper from a mutable reference to a writer.
    pub fn new(writer: &'w mut T) -> Self {
        Self { writer }
    }
}

impl<T: StdWrite> StdWrite for WriteWrapper<'_, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
