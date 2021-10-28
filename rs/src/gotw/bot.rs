use crate::{command::CommandMap, BotError, BotResult, TwitchIrcClient, User};
use log::{debug, info};
use std::env;
use std::fmt;

pub struct Bot {
    client: TwitchIrcClient,
    prefix: String,
    commands: CommandMap,
    config: BotConfig,
}

pub struct BotConfig {
    channel: String,
    username: String,
    password: String,
    prefix: String,
}

impl BotConfig {
    pub fn from_env() -> BotResult<Self> {
        Ok(Self {
            username: env::var("GOTW_NICK")?,
            password: env::var("GOTW_PASS")?,
            channel: env::var("GOTW_CHAN")?,
            prefix: env::var("GOTW_PREF")?,
        })
    }
}

#[derive(Debug)]
pub struct Message {
    pub words: Vec<String>,
    pub sender: User,
    pub raw: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message from {}: {}", self.sender.username(), self.raw,)
    }
}

impl Bot {
    pub fn login(cfg: BotConfig) -> BotResult<Self> {
        let r = Ok(Self {
            client: TwitchIrcClient::connect(
                cfg.username.clone(),
                cfg.password.clone(),
                cfg.channel.clone(),
            )?,
            prefix: cfg.prefix.clone(),
            commands: CommandMap::new(),
            config: cfg,
        });
        info!("Bot logged in");
        r
    }
    fn try_parse_message(&mut self) -> BotResult<Option<Message>> {
        let ret: BotResult<Option<Message>>;
        let message: String = self.client.get_message()?;
        if message.is_empty()
            || message
                == format!(
                    ":{name}!{name}@{name}.tmi.twitch.tv JOIN {}",
                    self.config.channel,
                    name = self.config.username
                )
        {
            return Ok(None);
        }
        let mut message = message.split(':');
        let tags = message.next().ok_or(BotError::MessageParse(
            "Could not get tags from twitch resonse".into(),
        ))?;
        let sender = message
            .next()
            .ok_or(BotError::MessageParse(
                "Could not parse username from twitch response.".into(),
            ))?
            .split('!')
            .next()
            .ok_or(BotError::MessageParse(
                "Could not parse username from twitch response".into(),
            ))?;
        let raw = message.collect::<Vec<&str>>().join(":");
        let user = User::parse(tags, sender);
        debug!("Message {:?} is command", raw);
        let mut words = raw.split(' ').map(|w| w.to_string()).collect();
		Ok(Some(Message {
	        words,
            sender: user,
            raw: raw.to_string(),
        }))
    }
    pub fn wait_commands(&mut self) -> BotResult<()> {
        let mut msg: Message;
        loop {
            if let Some(m) = self.try_parse_message()? {
                msg = m;
            } else {
                continue;
            }
        }
    }
}
