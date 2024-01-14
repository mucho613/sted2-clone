use recomposer_file::RcpFile;
use std::sync::Mutex;

pub struct FileState {
    pub file: Mutex<Option<Vec<u8>>>,
    pub song: Mutex<Option<RcpFile>>,
}
