pub struct Song {
    pub time_base: u32,
    pub events: Vec<Event>,
}

pub struct Event {
    pub delta_time: u32,
    pub event_body: EventBody,
}

pub enum EventBody {
    ChannelMessage(Vec<u8>),
    TempoChangeEvent(u32),
}
