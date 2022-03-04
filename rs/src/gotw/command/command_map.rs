use super::{
    commands::{Command, CommandPerms, NewCommand},
    CommandError::*
};
use crate::{bot::Message, BotError, BotResult};
use fasthash::{sea::Hash64, RandomState};
use log::{debug, info, warn, trace};
use std::collections::HashMap;
use std::str::FromStr;

pub struct CommandMap {
    triggers: HashMap<String, Command, RandomState<Hash64>>,
    commands: HashMap<String, Command, RandomState<Hash64>>,
    aliases: HashMap<String, Vec<String>, RandomState<Hash64>>,
}

impl CommandMap {
    // deserialize eventually
    pub fn new() -> Self {
        Self {
            triggers: HashMap::with_hasher(RandomState::new()),
            commands: HashMap::with_hasher(RandomState::new()),
            aliases: HashMap::with_hasher(RandomState::new()),
        }
    }
    pub fn try_run_cmd(&mut self, message: Message) -> BotResult<Option<String>> {
        let perms = CommandPerms::max(&message.sender);
        if perms >= CommandPerms::Mod {
            if message.raw.starts_with("!commands add") || message.raw.starts_with('+') {
                let to_add = NewCommand::from_str(&message.raw).unwrap();
                if to_add.cmd.is_trigger() {
                    info!("Trigger {} added by {}", to_add.name, message.sender.username());
                    let ret = Ok(Some(format!("Trigger {} successfully added", to_add.name)));
                    self.triggers.insert(to_add.name, to_add.cmd);
                    return ret;
                } else {
                    info!("Command {} added by {}", to_add.name, message.sender.username());
                    let ret = Ok(Some(format!("Command {} successfully added", to_add.name)));
                    self.commands.insert(to_add.name, to_add.cmd);
                    return ret;
                }
            } else if message.raw.starts_with("!commands del") || message.raw.starts_with('-') {
                let to_del = if message.raw.starts_with('-') {
                    &message.words[1]
                } else {
                    &message.words[2]
                };
                trace!("{}", to_del);
                if self.commands.contains_key(to_del) {
                    self.commands.remove(to_del);
                    info!("Command {} removed by {}", to_del, message.sender.username());
                    return Ok(Some(format!("Command {} successfully removed", to_del)));
                } else if self.triggers.contains_key(to_del) {
                    self.triggers.remove(to_del);
                    info!("Trigger {} removed by {}", to_del, message.sender.username());
                    return Ok(Some(format!("Trigger {} successfully removed", to_del)));
                } else {
                    warn!("Tried to remove a nonexistent item ({})", to_del);
                    return Err(BotError::Command(NotRegistered));
                }
            } else if message.raw.starts_with("!commands unalias") {
                let alias = &message.words[2];
                if self.aliases.contains_key(alias) {
                    info!("Alias {} removed by {}", alias, message.sender.username());
                    self.aliases.remove(alias);
                    return Ok(Some(format!(
                        "Alias {} successfully removed",
                        message.words[2]
                    )));
                } else {
                    warn!("Tried to remove a nonexistent alias ({})", alias);
                    return Err(BotError::Command(NotRegistered));
                }
            }
        }
        let cmd = if message.words[0].len() > 1 {
            let mut a = message.words[0].clone();
            a.remove(0);
            debug!("potential command name {}", a);
            a
        } else {
            "".into()
        };
        if message.words[0].starts_with('!') && self.commands.contains_key(&cmd) {
            let c = self.commands.get_mut(&cmd).unwrap();
            if perms >= c.perms() {
                info!("{} ran {}", message.sender.username(), message.words[0]);
                return c.run(message);
            } else {
                warn!("{} tried to run {}, insufficent permissions", message.sender.username(), message.words[0]);
                return Err(BotError::Command(InsufficientPerms));
            }
        } else {
            for (idx, word) in message.words.iter().enumerate() {
                if let Some(c) = self.triggers.get_mut(word) {
                    if perms >= c.perms() {
                        info!("{} ran trigger {}", message.sender.username(), message.words[idx]);
                        return c.run(message);
                    } else {
                        warn!("{} tried to run trigger {}, insufficent permissions", message.sender.username(), message.words[0]);
                        return Err(BotError::Command(InsufficientPerms));
                    }
                } else if self.aliases.contains_key(word) {
                    let mut rest = message.words[idx..].to_owned();
                    let mut all = self.aliases.get(word).unwrap().to_owned();
                    all.append(&mut rest);
                    return self.try_run_cmd(Message {
                        sender: message.sender,
                        words: all,
                        raw: message.words[idx..].join(" "),
                    });
                }
            }
        }
        Ok(None)
    }
}
