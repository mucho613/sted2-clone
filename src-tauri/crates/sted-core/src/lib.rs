mod play;
mod song_info;

pub use play::play;
use recomposer_file::RcpFile;

pub fn load(file_data: &[u8]) -> RcpFile {
    recomposer_file::parse(file_data)
}
