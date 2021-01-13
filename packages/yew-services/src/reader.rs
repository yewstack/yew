//! Service to load files using `FileReader`.

use crate::Task;
use anyhow::{anyhow, Result};
use gloo::events::EventListener;
use js_sys::Uint8Array;
use std::cmp;
use std::fmt;
#[doc(no_inline)]
pub use web_sys::{Blob, File};
use web_sys::{Event, FileReader};
use yew::callback::Callback;

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

impl ReaderService {
    /// Reads all bytes from a file and returns them with a callback.
    pub fn read_file(file: File, callback: Callback<FileData>) -> Result<ReaderTask> {
        let file_reader = FileReader::new().map_err(|_| anyhow!("couldn't acquire file reader"))?;
        let reader = file_reader.clone();
        let name = file.name();
        let callback = move |_event: &Event| {
            if let Ok(result) = reader.result() {
                let array = Uint8Array::new(&result);
                let data = FileData {
                    name,
                    content: array.to_vec(),
                };
                callback.emit(data);
            }
        };
        let listener = EventListener::once(&file_reader, "loadend", callback);
        file_reader.read_as_array_buffer(&file).unwrap();
        Ok(ReaderTask {
            file_reader,
            listener,
        })
    }

    /// Reads data chunks from a file and returns them with a callback.
    pub fn read_file_by_chunks(
        file: File,
        callback: Callback<Option<FileChunk>>,
        chunk_size: usize,
    ) -> Result<ReaderTask> {
        let file_reader = FileReader::new().map_err(|_| anyhow!("couldn't aquire file reader"))?;
        let name = file.name();
        let mut position = 0;
        let total_size = file.size() as usize;
        let reader = file_reader.clone();
        let callback = move |_event: &Event| {
            if let Ok(result) = reader.result() {
                if result.is_string() {
                    let started = FileChunk::Started { name: name.clone() };
                    callback.emit(Some(started));
                } else {
                    let array = Uint8Array::new_with_byte_offset(&result, 0);
                    let chunk = FileChunk::DataChunk {
                        data: array.to_vec(),
                        progress: position as f32 / total_size as f32,
                    };
                    callback.emit(Some(chunk));
                };
                // Read the next chunk
                if position < total_size {
                    let from = position;
                    let to = cmp::min(position + chunk_size, total_size);
                    position = to;
                    let blob = file.slice_with_i32_and_i32(from as _, to as _).unwrap();
                    if let Err(..) = reader.read_as_array_buffer(&blob) {
                        callback.emit(None);
                    }
                } else {
                    let finished = FileChunk::Finished;
                    callback.emit(Some(finished));
                }
            } else {
                callback.emit(None);
            }
        };
        let listener = EventListener::new(&file_reader, "loadend", callback);
        let blob = Blob::new().map_err(|_| anyhow!("Blob constructor is not supported"))?;
        file_reader.read_as_text(&blob).unwrap();
        Ok(ReaderTask {
            file_reader,
            listener,
        })
    }
}

/// A handle to control reading.
#[must_use = "the reader will abort when the task is dropped"]
pub struct ReaderTask {
    pub(super) file_reader: FileReader,
    #[allow(dead_code)]
    listener: EventListener,
}

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.file_reader.ready_state() == FileReader::LOADING
    }
}

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
