use recomposer_file::RcpFile;
use std::sync::Mutex;

pub struct FileState {
    pub rcpFile: Mutex<Option<RcpFile>>,
}
