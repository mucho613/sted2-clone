use recomposer_file::event::types::TrackEvent;

use super::{Measure, Song, Track};

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

            // events には TrackEvent::MeasureEnd を含めた全イベントが含まれている。
            // measures には、events を TrackEvent::MeasureEnd ごとに区切って格納する。
            let measures: Vec<Measure> = events
                .split(|event| matches!(event, TrackEvent::MeasureEnd))
                .map(|events| Measure {
                    events: events.to_vec(),
                    step_time: 0,
                })
                .collect();

            Track { measures }
        })
        .collect();

    Song { name, tracks }
}

// test of from_rcp_file
#[cfg(test)]
mod tests {
    use super::*;
    use recomposer_file::parse;

    #[test]
    fn test_from_rcp_file() {
        // Load file
        let test_file = include_bytes!("../test.rcp");
        let rcp_file = parse(test_file);

        // let rcp_file = RcpFile("../test.rcp").unwrap();
        let song = from_rcp_file(rcp_file);

        println!("{:#?}", song);
    }
}
