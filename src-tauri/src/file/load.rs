use std::{fs::File, io::Read};

use tauri::State;

use crate::{file::standard_midi_file::load::load, state::file_state::FileState};

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

    if smf.header_chunk.format != 0 {
        return Err("format 0 以外の SMF は再生できません。".to_string());
    }

    file_buffer
        .file
        .lock()
        .expect("Failed to lock file buffer")
        .replace(buffer);

    file_buffer
        .smf
        .lock()
        .expect("Failed to lock smf")
        .replace(smf);

    Ok(())
}
