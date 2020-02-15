//! Service to load files using `FileReader`.

use super::Task;
use crate::callback::Callback;
use std::cmp;
use std::fmt;
use stdweb::unstable::TryInto;
use stdweb::web::event::LoadEndEvent;
pub use stdweb::web::{Blob, File, IBlob};
use stdweb::web::{FileReader, FileReaderReadyState, FileReaderResult, IEventTarget, TypedArray};
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

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
        let file_reader = FileReader::new();
        let reader = file_reader.clone();
        let name = file.name();
        file_reader.add_event_listener(move |_event: LoadEndEvent| match reader.result() {
            Some(FileReaderResult::String(_)) => {
                unreachable!();
            }
            Some(FileReaderResult::ArrayBuffer(buffer)) => {
                let array: TypedArray<u8> = buffer.into();
                let data = FileData {
                    name: name.clone(),
                    content: array.to_vec(),
                };
                callback.emit(data);
            }
            None => {}
        });
        file_reader.read_as_array_buffer(&file).unwrap();
        ReaderTask { file_reader }
    }

    /// Reads data chunks from a file and returns them with a callback.
    pub fn read_file_by_chunks(
        &mut self,
        file: File,
        callback: Callback<FileChunk>,
        chunk_size: usize,
    ) -> ReaderTask {
        let file_reader = FileReader::new();
        let name = file.name();
        let mut position = 0;
        let total_size = file.len() as usize;
        let reader = file_reader.clone();
        file_reader.add_event_listener(move |_event: LoadEndEvent| {
            match reader.result() {
                // This branch is used to start reading
                Some(FileReaderResult::String(_)) => {
                    let started = FileChunk::Started { name: name.clone() };
                    callback.emit(started);
                }
                // This branch is used to send a chunk value
                Some(FileReaderResult::ArrayBuffer(buffer)) => {
                    let array: TypedArray<u8> = buffer.into();
                    let chunk = FileChunk::DataChunk {
                        data: array.to_vec(),
                        progress: position as f32 / total_size as f32,
                    };
                    callback.emit(chunk);
                }
                None => {}
            }
            // Read the next chunk
            if position < total_size {
                let file = &file;
                let from = position;
                let to = cmp::min(position + chunk_size, total_size);
                position = to;
                // TODO(#942): Implement `slice` method in `stdweb`
                let blob: Blob = (js! {
                    return @{file}.slice(@{from as u32}, @{to as u32});
                })
                .try_into()
                .unwrap();
                reader.read_as_array_buffer(&blob).unwrap();
            } else {
                let finished = FileChunk::Finished;
                callback.emit(finished);
            }
        });
        let blob: Blob = (js! {
            return (new Blob());
        })
        .try_into()
        .unwrap();
        file_reader.read_as_text(&blob).unwrap();
        ReaderTask { file_reader }
    }
}

/// A handle to control reading.
#[must_use]
pub struct ReaderTask {
    file_reader: FileReader,
}

impl fmt::Debug for ReaderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReaderTask")
    }
}

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.file_reader.ready_state() == FileReaderReadyState::Loading
    }
}

impl Drop for ReaderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.file_reader.abort();
        }
    }
}
