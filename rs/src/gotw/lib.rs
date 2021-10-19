mod bot;
mod command;
mod irc;
mod user;
pub use bot::{Bot, BotConfig};
pub(crate) use irc::TwitchIrcClient;
pub(crate) use user::User;
pub type BotResult<T> = Result<T, BotError>;

use std::fmt;

#[derive(Debug)]
pub enum BotError {
    Io(std::io::Error),
    MissingEnvironment(std::env::VarError),
    Ws(tungstenite::error::Error),
    MessageParse(String),
}
impl From<std::env::VarError> for BotError {
    fn from(e: std::env::VarError) -> Self {
        Self::MissingEnvironment(e)
    }
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
impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // thanks rust
        let s = match self {
            Self::Io(e) => e.to_string(),
            Self::MissingEnvironment(e) => e.to_string(),
            Self::Ws(e) => e.to_string(),
            Self::MessageParse(e) => e.to_string(),
        };
        write!(f, "{}", s)
    }
}
