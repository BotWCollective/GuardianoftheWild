use super::{
    commands::{Command as Cmd, CommandPerms},
    CommandError,
};
use crate::{bot::Message, BotError, BotResult};
use fasthash::{sea::Hash64, RandomState};
use log::{debug, info, warn};
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::str::FromStr;

use BotError::Command;
use CommandError::*;

pub struct CommandMap {
    keyword_re: Regex,
    keywords: HashMap<String, Cmd, RandomState<Hash64>>,
    commands: HashMap<String, Cmd, RandomState<Hash64>>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            keyword_re: Regex::new("").unwrap(),
            keywords: HashMap::with_hasher(RandomState::<Hash64>::new()),
            commands: HashMap::with_hasher(RandomState::<Hash64>::new()),
        }
    }
    pub fn lookup(&mut self, msg: Message) -> BotResult<Option<String>> {
        if msg.words.is_empty() || msg.raw.is_empty() {
            debug!("Message was empty!");
            return Ok(None);
        }
        let first = &msg.words[0];
        debug!("{:?}", first);
        debug!("{:?}", self.commands);
        if self.commands.contains_key(first) {
            info!("{} ran command {}", &msg.sender, first);
            self.commands.get_mut(first).unwrap().run(msg.sender)
        } else if first == "!commands" {
            if CommandPerms::max(&msg.sender) >= CommandPerms::Mod && msg.words.len() > 2 {
                match msg.words[1].as_str() {
                    "add" => {
                        if msg.words.len() >= 4 {
                            let cmd = Cmd::from_str(&msg.raw).expect("this shouldnt fail");
                            let name = msg
                                .raw
                                .split_ascii_whitespace()
                                .skip(2)
                                .skip_while(|w| w.starts_with('-'))
                                .next()
                                .unwrap_or("")
                                .to_string();
                            let add_to = if cmd.trigger() {
                                debug!("Adding keyword {}", name);
                                &mut self.keywords
                            } else {
                                debug!("Adding command {}", name);
                                &mut self.commands
                            };
                            let trigger = cmd.trigger();
                            if (cmd.case_sensitive() && add_to.contains_key(&name))
                                || add_to.contains_key(&name.to_ascii_lowercase())
                            {
	                            warn!("Tried to register command {} that already exists", name);
                                Err(Command(AlreadyRegistered))
                            } else {
                                if cmd.case_sensitive() {
	                                debug!("inserted case sensitive");
                                    add_to.insert(name.clone(), cmd);
                                } else {
	                                debug!("inserted lowercase");
                                    add_to.insert(name.to_ascii_lowercase(), cmd);
                                }
                                if trigger {
		                            self.refresh_keywords();
                                }
                                Ok(Some(format!("Command {} successfully added", name)))
                            }
                        } else {
                            Err(Command(NotEnoughArgs))
                        }
                    }
                    "alias" => {
                        if msg.words.len() >= 4 {
                            debug!("Aliasing {} to {}", msg.words[2], msg.words[3]);
                            self.alias(
                                &msg.words[2],
                                &msg.words[3],
                                if let Some(w) = msg.words.get(4) {
                                    debug!("Aliasing as keyword: {}", w == "-k");
                                    w == "-k"
                                } else {
                                    debug!("Aliasing as keyword: false");
                                    false
                                },
                            )
                        } else {
                            Err(Command(NotEnoughArgs))
                        }
                    }
                    "del" => {
                        if msg.words.len() >= 3 {
                            debug!("Deleting {}", &msg.words[2]);
                            self.delete(&msg.words[2])
                        } else {
                            Err(Command(NotEnoughArgs))
                        }
                    }
                    "show" => Ok(None),
                    _ => Ok(None),
                }
            } else {
                Ok(Some("this will be the list of commands eventually".into()))
            }
        } else {
            let keywords: Vec<_> = self
                .keyword_re
                .find_iter(&msg.raw)
                .map(|m| m.as_str().to_string())
                .collect();
            let keyword = keywords.iter().max().unwrap();
            debug!("found keyword: {}", keyword);
            if self.keywords.contains_key(keyword) {
	            info!("{} said keyword {}", msg.sender, keyword);
                self.keywords.get_mut(keyword).unwrap().run(msg.sender)
            } else {
                Err(Command(NotFound))
            }
        }
    }
    fn alias(
        &mut self,
        target: &str,
        alias: &str,
        target_keyword: bool,
    ) -> BotResult<Option<String>> {
        if self.commands.get(target).is_some() {
	        let cmd = self.commands.get(target).unwrap().clone();
		    let add_to = if target_keyword {&mut self.keywords} else {&mut self.commands};
            if !add_to.contains_key(alias) {
				add_to.insert(alias.to_string(), cmd);
            } else {
				warn!("tried to alias to an existing command: {}", alias);
	            return Err(Command(AlreadyRegistered));
            }
            Ok(Some(format!("{} aliased to {}", target, alias)))
        } else if self.keywords.get(target).is_some() {
	        let cmd = self.keywords.get(target).unwrap().clone();
		    let add_to = if target_keyword {&mut self.keywords} else {&mut self.commands};
            if !add_to.contains_key(alias) {
				add_to.insert(alias.to_string(), cmd);
            } else {
				warn!("tried to alias to an existing command: {}", alias);
	            return Err(Command(AlreadyRegistered));
            }
            Ok(Some(format!("{} aliased to {}", target, alias)))
        } else {
            Err(Command(NotRegistered))
        }
    }
    fn delete(&mut self, target: &str) -> BotResult<Option<String>> {
        if self.commands.remove(target).is_some() {
	        debug!("removed command {}", target);
            Ok(Some(format!("Command {} removed", target)))
        } else if self.keywords.remove(target).is_some() {
	        debug!("removed keyword {}", target);
            self.refresh_keywords();
            Ok(Some(format!("Keyword {} removed", target)))
        } else {
	        warn!("Tried to delete command that doesnt exist: {}", target);
            Err(Command(NotRegistered))
        }
    }
    fn refresh_keywords(&mut self) {
        let joined_keywords = self
            .keywords
            .keys()
            .map(|t| regex::escape(t))
            .collect::<Vec<_>>()
            .join("|");
        let re_str = format!(r"\b({})\b", joined_keywords);
        let re_build = RegexBuilder::new(&re_str);
        self.keyword_re = re_build.build().unwrap();
        debug!("Refreshed keyword regex: {:?}", self.keyword_re.as_str());
    }
}
