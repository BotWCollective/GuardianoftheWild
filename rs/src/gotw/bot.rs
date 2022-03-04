use crate::{
    command::{CommandError, CommandMap},
    BotError, BotResult, TwitchIrcClient, User,
};
use log::{debug, info};
use std::env;
use std::fmt;

pub struct Bot {
    client: TwitchIrcClient,
    commands: CommandMap,
    config: BotConfig,
}

pub struct BotConfig {
    channel: String,
    username: String,
    password: String,
}

impl BotConfig {
    pub fn from_env() -> BotResult<Self> {
        Ok(Self {
            username: env::var("GOTW_NICK")?,
            password: env::var("GOTW_PASS")?,
            channel: env::var("GOTW_CHAN")?,
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
            commands: CommandMap::new(),
            config: cfg,
        });
        info!("Bot logged in");
        r
    }
    fn try_parse_message(&mut self) -> BotResult<Option<Message>> {
        let message = self.client.get_message()?;
        if message.is_empty()
            || message
                == format!(
                    ":{name}!{name}@{name}.tmi.twitch.tv JOIN {}",
                    self.config.channel,
                    name = self.config.username
                )
        {
            debug!("Message thrown out");
            return Ok(None);
        }
        let message = message.trim_end_matches("\r\n");
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
        debug!("Got username: {}", sender);
        let raw = message.collect::<Vec<&str>>().join(":");
        let user = User::parse(tags, sender);
        let words = raw.split(' ').map(|w| w.to_string()).collect();
        Ok(Some(Message {
            words,
            sender: user,
            raw: raw.to_string(),
        }))
    }
    pub fn wait_commands(&mut self) -> BotResult<()> {
        use BotError::Command;
        loop {
            if let Some(m) = self.try_parse_message()? {
                info!("{}: {}", m.sender, m.raw);
                let res = self.commands.try_run_cmd(m);
                match res {
                    Err(Command(CommandError::NotEnoughArgs)) => {
                        self.client.send_message("Not enough arguments!")?;
                    }
                    Err(Command(CommandError::NotRegistered)) => {
                        self.client.send_message("Command/trigger/alias does not exist!")?;
                    }
                    Ok(Some(ref m)) => {
                        info!("sent: {}", m);
                        self.client.send_message(&m)?;
                    }
                    _ => {}
                }
            }
        }
    }
}
