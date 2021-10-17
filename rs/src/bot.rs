use crate::{BotError, BotResult, TwitchIrcClient};
use log::{debug, info};
use std::collections::HashMap;
use std::time::Instant;
use std::env;

pub struct Bot {
    client: TwitchIrcClient,
    prefix: String,
    // string to string for now until command maps are sorted out
    commands: HashMap<String, String>,
    timers: [Instant; 2],
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
    pub sender: String,
    pub raw: String,
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
            timers: [Instant::now(), Instant::now()],
            config: cfg,
        });
        info!("Bot logged in");
        r
    }
    pub fn try_parse_message(&mut self) -> BotResult<Option<Message>> {
        let mut ret = Ok(None);
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
        message.next();
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
        if !raw.starts_with(&self.prefix) {
            debug!("Message {:?} is not command", raw);
            println!("{}", self.timers[0].elapsed().as_secs());
            ret = Ok(Some(Message {
                command: None,
                args: None,
                sender: sender.to_string(),
                raw: raw.to_string(),
            }));
        } else {
	        debug!("Message {:?} is command", raw);
            let mut split = raw.split(' ');
            let command = split.next().unwrap().split_once(&self.prefix).unwrap().1;
            let command = &command[..command.len() - 2];
            ret = Ok(Some(Message {
                command: Some(command.to_string()),
                args: None,
                sender: sender.to_string(),
                raw: raw.to_string(),
            }));
        }
        ret
    }
}
