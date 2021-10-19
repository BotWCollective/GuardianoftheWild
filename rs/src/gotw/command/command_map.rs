use super::commands::{Command, CommandPerms};
use crate::bot::Message;
use std::collections::HashMap;

pub struct CommandMap {
    map: HashMap<String, Command>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn run_command(&mut self, msg: Message) -> Option<String> {
        if msg.command.is_none() {
            return None;
        }
        if let Some(args) = msg.args {
            if CommandPerms::Mod <= CommandPerms::max(&msg.sender) {
                match msg.command.unwrap().as_str() {
                    "!commands" if args.len() == 2 => match args[0].as_str() {
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
                    },
                    "!alias" if args.len() == 2 => {
                        self.alias(&args[0], &args[1]);
                        None
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn insert(&mut self, name: String, command: Command) {
        self.map.insert(name, command);
    }
    pub fn alias(&mut self, target: &str, alias: &str) {
        if let Some(c) = self.map.get(target) {
            self.insert(alias.to_string(), c.clone());
        }
    }
}
