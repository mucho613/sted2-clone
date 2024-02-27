use std::{fs::File, io::Read};

use recomposer_file::parse;
use tauri::State;

use crate::song::from_rcp_file::from_rcp_file;
use crate::state::file_state::FileState;
use crate::state::song_state::SongState;

#[tauri::command]
pub fn load_file(file_path: String, song_state: State<'_, SongState>) -> Result<(), String> {
    let file = File::open(file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()),
    };

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let rcp_file = parse(&buffer);

    // TODO: ここで Song に変換
    let song = from_rcp_file(rcp_file);

    song_state
        .song
        .lock()
        .expect("Failed to lock smf")
        .replace(song);

    Ok(())
}
