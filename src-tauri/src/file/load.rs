use std::{fs::File, io::Read};

use tauri::State;

use crate::state::FileBuffer;

#[tauri::command]
pub fn load_file(file_path: String, file_buffer: State<'_, FileBuffer>) -> Result<(), String> {
    let file = File::open(file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    *file_buffer.file.lock().unwrap() = buffer;
    Ok(())
}
