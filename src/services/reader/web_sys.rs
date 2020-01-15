//! `web-sys` implementation for the reader service.

use crate::callback::Callback;
use crate::services::Task;
use gloo::events::EventListener;
use js_sys::Uint8Array;
use std::cmp;
use std::fmt;
pub use web_sys::{Blob, File};
use web_sys::{Event, FileReader};

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
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Reads all bytes from a file and returns them with a callback.
    pub fn read_file(&mut self, file: File, callback: Callback<FileData>) -> ReaderTask {
        let file_reader = FileReader::new().unwrap();
        let reader = file_reader.clone();
        let name = file.name();
        let callback = move |_event: &Event| {
            let array = Uint8Array::new_with_byte_offset(
                &reader
                    .result()
                    .expect("`FileReader` hasn't finished loading"),
                0,
            );
            let data = FileData {
                name: name.clone(),
                content: array.to_vec(),
            };
            callback.emit(data);
        };
        let listener = Some(EventListener::new(&file_reader, "loadend", callback));
        file_reader.read_as_array_buffer(&file).unwrap();
        ReaderTask {
            file_reader,
            listener,
        }
    }

    /// Reads data chunks from a file and returns them with a callback.
    pub fn read_file_by_chunks(
        &mut self,
        file: File,
        callback: Callback<FileChunk>,
        chunk_size: usize,
    ) -> ReaderTask {
        let file_reader = FileReader::new().unwrap();
        let name = file.name();
        let mut position = 0;
        let total_size = file.size() as usize;
        let reader = file_reader.clone();
        let callback = move |_event: &Event| {
            let result = reader
                .result()
                .expect("`FileReader` hasn't finished loading");

            if result.is_string() {
                let started = FileChunk::Started { name: name.clone() };
                callback.emit(started);
            } else {
                let array = Uint8Array::new_with_byte_offset(&result, 0);
                let chunk = FileChunk::DataChunk {
                    data: array.to_vec(),
                    progress: position as f32 / total_size as f32,
                };
                callback.emit(chunk);
            };
            // Read the next chunk
            if position < total_size {
                let from = position;
                let to = cmp::min(position + chunk_size, total_size);
                position = to;
                let blob = file.slice_with_i32_and_i32(from as _, to as _).unwrap();
                reader.read_as_array_buffer(&blob).unwrap();
            } else {
                let finished = FileChunk::Finished;
                callback.emit(finished);
            }
        };
        let listener = Some(EventListener::new(&file_reader, "loadend", callback));
        let blob = Blob::new().unwrap();
        file_reader.read_as_text(&blob).unwrap();
        ReaderTask {
            file_reader,
            listener,
        }
    }
}

/// A handle to control reading.
#[must_use]
pub struct ReaderTask {
    file_reader: FileReader,
    listener: Option<EventListener>,
}

impl fmt::Debug for ReaderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReaderTask")
    }
}

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.file_reader.ready_state() == FileReader::LOADING
    }

    fn cancel(&mut self) {
        self.file_reader.abort();
        drop(
            self.listener
                .take()
                .expect("tried to cancel websocket twice"),
        )
    }
}

impl Drop for ReaderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
