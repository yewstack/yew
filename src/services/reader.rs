//! Service to load files using `FileReader`.

use crate::services::Task;
use std::fmt;
cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
        pub use std_web::*;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
        pub use self::web_sys::*;
    }
}

/// Struct that represents data of a file.
#[derive(Clone, Debug)]
pub struct FileData {
    /// Name of loaded file.
    pub name: String,
    /// Content of loaded file.
    pub content: Vec<u8>,
}

/// Struct that represents a chunk of a file.
#[derive(Clone, Debug)]
pub enum FileChunk {
    /// Reading of chunks started. Equals **0%** progress.
    Started {
        /// Name of loaded file.
        name: String,
    },
    /// The next data chunk that read. Also provides a progress value.
    DataChunk {
        /// The chunk of binary data.
        data: Vec<u8>,
        /// The progress value in interval: `0 < progress <= 1`.
        progress: f32,
    },
    /// Reading of chunks finished. Equals **100%** progress.
    Finished,
}

/// A reader service attached to a user context.
#[derive(Default, Debug)]
pub struct ReaderService {}

impl fmt::Debug for ReaderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReaderTask")
    }
}

impl Drop for ReaderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.file_reader.abort();
        }
    }
}
