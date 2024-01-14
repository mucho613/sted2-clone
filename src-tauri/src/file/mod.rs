use std::{fs::File, io::Read};

use recomposer_file::parse;
use tauri::State;

use crate::state::file_state::FileState;

#[tauri::command]
pub fn load_file(file_path: String, file_buffer: State<'_, FileState>) -> Result<(), String> {
    let file = File::open(file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    // RCP ファイルとしてパースして、格納する
    let song = parse(&buffer);
    file_buffer
        .file
        .lock()
        .expect("Failed to lock file buffer")
        .replace(buffer);

    file_buffer
        .song
        .lock()
        .expect("Failed to lock smf")
        .replace(song);

    Ok(())
}
