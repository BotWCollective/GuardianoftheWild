use crate::{BotError, BotResult, TwitchIrcClient, User};
use log::{debug, info};
use std::collections::HashMap;
use std::env;
use std::fmt;

pub struct Bot {
    client: TwitchIrcClient,
    prefix: String,
    // string to string for now until command maps are sorted out
    commands: HashMap<String, String>,
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
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub sender: User,
    pub raw: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Message from {}: {} {}",
            self.sender.username(),
            self.raw,
            if let Some(c) = self.command.as_ref() {
                c.to_string()
            } else {
                String::new()
            }
        )
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
            commands: HashMap::new(),
            config: cfg,
        });
        info!("Bot logged in");
        r
    }
    pub fn try_parse_message(&mut self) -> BotResult<Option<Message>> {
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
        if !raw.starts_with(&self.prefix) {
            debug!("Message {:?} is not command", raw);
            ret = Ok(Some(Message {
                command: None,
                args: None,
                sender: user,
                raw: raw.to_string(),
            }));
        } else {
            debug!("Message {:?} is command", raw);
            let mut split = raw.split(' ');
            let command = split.next().unwrap().split_once(&self.prefix).unwrap().1;
            let command = &command[..command.len() - 2];
            let mut args = vec![];
			while let Some(a) = split.next() {
				args.push(a.to_string());
			}
            ret = Ok(Some(Message {
                command: Some(command.to_string()),
                args: if args.is_empty() {None} else {Some(args)},
                sender: user,
                raw: raw.to_string(),
            }));
        }
        ret
    }
}
