pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

pub const RADIO_PORT: u16 = 8888;
pub const SERVER_PORT: u16 = 8889;