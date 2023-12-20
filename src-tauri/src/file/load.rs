use std::{fs::File, io::Read};

use tauri::State;

use crate::{song::convert::convert, state::FileState};

#[tauri::command]
pub fn load_file(file_path: String, file_buffer: State<'_, FileState>) -> Result<(), String> {
    let file = File::open(file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    println!("変換開始");
    let song = convert(&buffer);
    println!("変換終了");

    *file_buffer.file.lock().unwrap() = buffer;

    *file_buffer.song.lock().unwrap() = Some(song);

    Ok(())
}
