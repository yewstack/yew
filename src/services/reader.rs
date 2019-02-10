//! Service to load files using `FileReader`.

pub use stdweb::web::File;
use stdweb::web::{
    IEventTarget,
    FileReader,
    FileReaderReadyState,
    FileReaderResult,
    TypedArray,
};
use stdweb::web::event::{
    LoadEndEvent,
};
use callback::Callback;
use super::Task;

/// Struct that represents data of file.
#[derive(Clone, Debug)]
pub struct FileData {
    /// Name of loaded file.
    pub name: String,
    /// Content of loaded file.
    pub content: Vec<u8>,
}

/// A reader service attached to a user context.
#[derive(Default)]
pub struct ReaderService {}

impl ReaderService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Reads all bytes from files and returns them with a callback.
    pub fn read_file(&mut self, file: File, callback: Callback<FileData>) -> ReaderTask {
        let file_reader = FileReader::new();
        let reader = file_reader.clone();
        let name = file.name();
        file_reader.add_event_listener(move |event: LoadEndEvent| {
            match reader.result() {
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
                None => { }
            }
        });
        file_reader.read_as_array_buffer(&file).unwrap();
        ReaderTask {
            file_reader,
        }
    }
}

/// A handle to control reading.
#[must_use]
pub struct ReaderTask {
    file_reader: FileReader,
}

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.file_reader.ready_state() == FileReaderReadyState::Loading
    }

    fn cancel(&mut self) {
        self.file_reader.abort();
    }
}

impl Drop for ReaderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
