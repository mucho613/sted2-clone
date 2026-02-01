use encoding_rs::SHIFT_JIS;
use recomposer_file::RcpFile;

pub fn display_song_info(rcp_file: &RcpFile) {
    // 曲タイトルを表示
    let title_bytes = &rcp_file.header_block.title;
    let (decoded, _, _) = SHIFT_JIS.decode(title_bytes);
    println!("Title: {}", decoded);

    // 曲メモを表示
    // 曲メモは 28 bytes で1行が構成され、計 12 行、合計 336 bytes となっているため、
    // 28 bytes ずつデコードし、改行を挿入して表示する
    println!("Memo:");
    let memo_bytes = &rcp_file.header_block.memo;
    for i in 0..12 {
        let start = i * 28;
        let end = start + 28;
        let (decoded, _, _) = SHIFT_JIS.decode(&memo_bytes[start..end]);
        println!("{}", decoded);
    }

    // 各トラックのコメントを表示
    for track in &rcp_file.track_block.tracks {
        let comment_bytes = &track.track_header.comment;
        let (decoded, _, _) = SHIFT_JIS.decode(comment_bytes);

        // トラック番号
        println!("Track Number: {:#2}", track.track_header.track_number);
        println!("Comment: {}", decoded);
    }

    // リズムパートのパート名を表示
    for rhythm_note in &rcp_file.header_block.rhythm_notes {
        let name_bytes = &rhythm_note.name;
        let (decoded, _, _) = SHIFT_JIS.decode(name_bytes);
        println!("Rhythm Name: {}", decoded);
    }

    // ユーザーエクスクルーシブ
    for ue in &rcp_file.header_block.user_exclusives {
        let data_bytes = &ue.memo;
        let (decoded, _, _) = SHIFT_JIS.decode(data_bytes);
        println!("User Exclusive Data: {}", decoded);
    }

    // トラック2のイベントを全て表示（デバッグ用）
    for event in rcp_file.track_block.tracks[1].track_events.iter().take(100) {
        println!("Event: {:?}", event);
    }
}
