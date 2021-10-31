use crate::user::User;
use super::CommandError;
use crate::{BotResult, BotError};

use std::cmp::Ordering;

#[derive(Clone)]
pub struct Command {
    perms: CommandPerms,
    trigger: Option<TriggerInfo>,
    action: CommandAction,
    runs: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub enum CommandPerms {
    Anyone,
    Vip,
    Mod,
    Broadcaster,
}

use CommandPerms::*;

impl CommandPerms {
    pub fn max(user: &User) -> Self {
        if user
            .tag("badges")
            .unwrap_or(&"".to_string())
            .contains("broadcaster")
        {
            Broadcaster
        } else if user.tag("mod").unwrap_or(&"".to_string()) == "1" {
            Mod
        } else if user
            .tag("badges")
            .unwrap_or(&"".to_string())
            .contains("vip")
        {
            Vip
        } else {
            Anyone
        }
    }
}

#[derive(Clone)]
pub enum CommandAction {
    Static { ret: String },
    Template { template: String },
    Script { command: String, file: String },
}

impl CommandAction {
	fn run(&self) -> BotResult<Option<String>> {
        match &self {
            CommandAction::Static { ret } => Ok(Some(ret.into())),
            // do something eventually
            _ => Ok(Some("".into())),
        }

	}
}

impl Command {
    pub fn new(perms: CommandPerms, action: CommandAction) -> Self {
        Self {
            perms,
            action,
            runs: 0,
        }
    }
    pub fn run(&mut self, user: User) -> BotResult<Option<String>> {
        if self.perms > CommandPerms::max(&user) {
            return Err(BotError::Command(CommandError::InsufficientPerms));
        }
        self.runs += 1;
        self.action.run()
    }
}

#[derive(Clone)]
pub struct TriggerInfo {
	priority: usize,
	// probably more here later
}

impl TriggerInfo {
	pub fn new(priority: usize) -> Self {
		Self {
			priority,
		}
	}
}

impl PartialEq for TriggerInfo {
	fn eq(&self, other: &Self) -> bool {
		self.priority == other.priority
	}
}
impl Eq for TriggerInfo {}
impl PartialOrd for TriggerInfo {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.priority.cmp(&other.priority))
	}
}
impl Ord for TriggerInfo {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

