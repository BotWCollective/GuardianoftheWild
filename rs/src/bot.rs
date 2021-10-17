use crate::{BotError, BotResult, TwitchIrcClient};
use log::{debug, info};
use std::collections::HashMap;
use std::time::Instant;

pub struct Bot {
    client: TwitchIrcClient,
    prefix: String,
    // string to string for now until command maps are sorted out
    commands: HashMap<String, String>,
    timers: [Instant; 2],
    config: BotConfig,
}

pub struct BotConfig {
    pub channel: String,
    pub username: String,
    pub password: String,
    pub prefix: String,
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
            if self.timers[0].elapsed().as_secs() >= 2400 {
                self.client.send_message("Big thanks to LeftShark_Vevo for hosting this! Be sure to follow the runners in their own channels! | twitch.tv/leftshark_vevo ^_^ twitch.tv/vladisvrau ^_^ twitch.tv/coensi ^_^ twitch.tv/itntpiston |")?;
                self.timers[0] = Instant::now();
            } else if self.timers[1].elapsed().as_secs() >= 3600 {
                self.client.send_message("Everyone is free to be comms <3 pls join Shark Squad if you want to help out! https://discord.gg/FkZuJaJ9eE")?;
                self.timers[1] = Instant::now();
            }
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
            if command == "runners" {
                self.client.send_message("Big thanks to LeftShark_Vevo for hosting this! Be sure to follow the runners in their own channels! | twitch.tv/leftshark_vevo ^_^ twitch.tv/vladisvrau ^_^ twitch.tv/coensi ^_^ twitch.tv/itntpiston |")?;
                self.timers[0] = Instant::now();
            } else if command == "host" {
                self.client.send_message("Everyone is free to be comms <3 pls join Shark Squad if you want to help out! https://discord.gg/FkZuJaJ9eE")?;
                self.timers[1] = Instant::now();
            }
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
