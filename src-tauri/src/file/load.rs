use std::{fs::File, io::Read};

use tauri::State;

use crate::{file::standard_midi_file::load::load, state::FileState};

#[tauri::command]
pub fn load_file(file_path: String, file_buffer: State<'_, FileState>) -> Result<(), String> {
    let file = File::open(file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    // SMF として格納する
    let smf = load(&buffer).expect("Failed to parse SMF");

    *file_buffer.file.lock().unwrap() = buffer;

    *file_buffer.smf.lock().unwrap() = Some(smf);

    Ok(())
}
