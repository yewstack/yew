//! `stdweb` implementation for the reader service.

use super::*;
use crate::callback::Callback;
use crate::services::Task;
use std::cmp;
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::web::event::LoadEndEvent;
pub use stdweb::web::{Blob, File, IBlob};
use stdweb::web::{FileReader, FileReaderReadyState, FileReaderResult, IEventTarget, TypedArray};
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

fn new_file_reader() -> Result<FileReader, &'static str> {
    let file_reader = js! {
        try {
            return new FileReader;
        } catch(error) {
            return error;
        }
    };
    FileReader::try_from(js!( return @{file_reader.as_ref()}; ))
        .map_err(|_| "couldn't aquire file reader")
}

impl ReaderService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Reads all bytes from a file and returns them with a callback.
    pub fn read_file(
        &mut self,
        file: File,
        callback: Callback<FileData>,
    ) -> Result<ReaderTask, &str> {
        let file_reader = new_file_reader()?;
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
        Ok(ReaderTask { file_reader })
    }

    /// Reads data chunks from a file and returns them with a callback.
    pub fn read_file_by_chunks(
        &mut self,
        file: File,
        callback: Callback<Option<FileChunk>>,
        chunk_size: usize,
    ) -> Result<ReaderTask, &str> {
        let file_reader = new_file_reader()?;
        let name = file.name();
        let mut position = 0;
        let total_size = file.len() as usize;
        let reader = file_reader.clone();
        file_reader.add_event_listener(move |_event: LoadEndEvent| {
            if let Some(result) = reader.result() {
                match result {
                    // This branch is used to start reading
                    FileReaderResult::String(_) => {
                        let started = FileChunk::Started { name: name.clone() };
                        callback.emit(Some(started));
                    }
                    // This branch is used to send a chunk value
                    FileReaderResult::ArrayBuffer(buffer) => {
                        let array: TypedArray<u8> = buffer.into();
                        let chunk = FileChunk::DataChunk {
                            data: array.to_vec(),
                            progress: position as f32 / total_size as f32,
                        };
                        callback.emit(Some(chunk));
                    }
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
                    callback.emit(Some(finished));
                }
            } else {
                callback.emit(None);
            }
        });
        let blob: Blob = (js! {
            return (new Blob());
        })
        .try_into()
        .unwrap();
        file_reader.read_as_text(&blob).unwrap();
        Ok(ReaderTask { file_reader })
    }
}

/// A handle to control reading.
#[must_use]
pub struct ReaderTask {
    pub(super) file_reader: FileReader,
}

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.file_reader.ready_state() == FileReaderReadyState::Loading
    }
}
