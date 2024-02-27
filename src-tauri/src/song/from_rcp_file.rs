use super::{Song, Track};

/// RCP ファイルから Song を生成
pub fn from_rcp_file(rcp_file: recomposer_file::RcpFile) -> Song {
    // let name = rcp_file.header_block.title;
    let name = "Test song name".to_string();
    let tracks = rcp_file
        .track_block
        .tracks
        .into_iter()
        .map(|track| {
            let events = track.track_events;
            Track { events }
        })
        .collect();

    Song { name, tracks }
}
