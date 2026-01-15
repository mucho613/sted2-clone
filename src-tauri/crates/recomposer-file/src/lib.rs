use header_block::types::HeaderBlock;
use track_block::types::TrackBlock;

pub mod event;
pub mod header_block;
pub mod track_block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcpFile {
    pub header_block: HeaderBlock,
    pub track_block: TrackBlock,
}

pub fn parse(i: &[u8]) -> RcpFile {
    let (i, header_block) =
        header_block::parse::parse_header_block(i).expect("Failed to parse header block");

    let (_, track_block) =
        track_block::parse::parse_track_block(i).expect("Failed to parse track block");

    RcpFile {
        header_block,
        track_block,
    }
}
