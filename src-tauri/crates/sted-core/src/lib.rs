mod play;
mod ports;
mod song_info;

pub use play::{OutputTarget, PlayError, play};
pub use ports::{MidiPortsError, SerialPortsError, midi_output_ports, serial_ports};
use recomposer_file::RcpFile;

pub fn load(file_data: &[u8]) -> RcpFile {
    recomposer_file::parse(file_data)
}
