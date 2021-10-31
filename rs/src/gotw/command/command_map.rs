use super::{
    commands::{Command as Cmd, CommandPerms},
    CommandError,
};
use crate::{bot::Message, BotError, BotResult};
use fasthash::{sea::Hash64, RandomState};
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

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
            return Ok(None);
        }
        if self.commands.contains_key(&msg.words[0]) {
            self.commands.get(&msg.words[0]).unwrap().run(msg.sender)
        } else if msg.words[0] == "!commands" {
            if CommandPerms::max(&msg.sender) >= CommandPerms::Mod && msg.words.len() > 2 {
                match msg.words[1].as_str() {
                    "add" => Ok(None),
                    "alias" => {
                        if msg.words.len() >= 4 {
                            self.alias(
                                &msg.words[2],
                                &msg.words[3],
                                if let Some(w) = msg.words.get(4) {
                                    w == "-k"
                                } else {
                                    false
                                },
                            )
                        } else {
                            Err(Command(NotEnoughArgs))
                        }
                    }
                    "del" => {
                        if msg.words.len() >= 3 {
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
            if self.keywords.contains_key(keyword) {
                return self.keywords.get(keyword).unwrap().run(msg.sender);
            }
            Ok(None)
        }
    }
    fn alias(
        &mut self,
        target: &str,
        alias: &str,
        target_keyword: bool,
    ) -> BotResult<Option<String>> {
        if self.commands.get(target).is_some() {
            if target_keyword {
                self.keywords.insert(
                    alias.to_string(),
                    self.commands.get(target).unwrap().clone(),
                );
                self.refresh_keywords();
            } else {
                self.commands.insert(
                    alias.to_string(),
                    self.commands.get(target).unwrap().clone(),
                );
            }
            Ok(Some(format!("{} aliased to {}", target, alias)))
        } else if self.keywords.get(target).is_some() {
            if target_keyword {
                self.keywords.insert(
                    alias.to_string(),
                    self.commands.get(target).unwrap().clone(),
                );
                self.refresh_keywords();
            } else {
                self.commands.insert(
                    alias.to_string(),
                    self.commands.get(target).unwrap().clone(),
                );
            }
            Ok(Some(format!("{} aliased to {}", target, alias)))
        } else {
            Err(Command(NotFound))
        }
    }
    fn delete(&mut self, target: &str) -> BotResult<Option<String>> {
        if self.commands.remove(target).is_some() {
            Ok(Some(format!("Command {} removed", target)))
        } else if self.keywords.remove(target).is_some() {
            self.refresh_keywords();
            Ok(Some(format!("Keyword {} removed", target)))
        } else {
            Err(Command(NotFound))
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
    }
}
