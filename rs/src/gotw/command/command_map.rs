use super::{
    commands::{Command, CommandPerms},
    CommandError,
};
use crate::{bot::Message, BotError, BotResult};
use fasthash::{sea::Hash64, RandomState};
use regex::{Regex, RegexBuilder};
use std::collections::{HashMap, HashSet};

pub struct CommandMap {
    trigger_re: Regex,
    triggers: HashSet<String, RandomState<Hash64>>,
    map: HashMap<String, Command, RandomState<Hash64>>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            trigger_re: Regex::new("").unwrap(),
            triggers: HashSet::with_hasher(RandomState::<Hash64>::new()),
            map: HashMap::with_hasher(RandomState::<Hash64>::new()),
        }
    }
    pub fn run_command(&mut self, msg: Message) -> Option<String> {
        if let Some(args) = msg.args {
            let perms = CommandPerms::max(&msg.sender);
            match msg.command.unwrap().as_str() {
                "!commands" if args.len() == 2 && CommandPerms::Mod <= perms => {
                    match args[0].as_str() {
                        "remove" => {
                            let res = self.map.remove(&args[1]);
                            if res.is_some() {
                                Some(format!("Command {} removed.", args[1]))
                            } else {
                                Some(format!("Command {} does not exist!", args[1]))
                            }
                        }
                        "show" => {
                            if let Some(_) = self.map.get(&args[1]) {
                                Some(format!("{}: command show as yet unimplemented", args[1]))
                            } else {
                                Some(format!("Command {} does not exist!", args[1]))
                            }
                        }
                        _ => None,
                    }
                }
                "!alias" if args.len() == 2 && CommandPerms::Mod <= perms => {
                    self.alias(&args[0], &args[1]);
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn alias(&mut self, target: &str, alias: &str) {
        if self.map.get(target).is_some() {
            self.map
                .insert(alias.to_string(), self.map.get(target).unwrap().clone());
        }
    }
    fn register_trigger(&mut self, trigger: &str) -> BotResult<()> {
        if self.triggers.get(trigger).is_some() {
            return Err(BotError::Command(CommandError::AlreadyRegistered));
        }
        self.triggers.insert(trigger.to_owned());
        self.refresh_triggers();
        Ok(())
    }
    fn unregister_trigger(&mut self, trigger: &str) -> BotResult<()> {
        if !self.triggers.remove(trigger) {
            return Err(BotError::Command(CommandError::NotFound));
        }
        self.refresh_triggers();
        Ok(())
    }
    fn refresh_triggers(&mut self) {
        let joined_triggers = self
            .triggers
            .iter()
            .map(|t| regex::escape(t))
            .collect::<Vec<_>>()
            .join("|");
        let re_str = format!(r"\b({})\b", joined_triggers);
        let re_build = RegexBuilder::new(&re_str);
        self.trigger_re = re_build.build().unwrap();
    }
}
