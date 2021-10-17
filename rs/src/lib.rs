mod bot;
mod irc;
mod log;
pub use bot::{Bot, BotConfig};
pub(crate) use irc::TwitchIrcClient;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Debug)]
pub enum BotError {
    Io(std::io::Error),
    Ws(tungstenite::error::Error),
    MessageParse(String),
}

impl From<std::io::Error> for BotError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<tungstenite::error::Error> for BotError {
    fn from(e: tungstenite::error::Error) -> Self {
        Self::Ws(e)
    }
}
