use crate::user::User;
use super::CommandError;
use crate::{BotResult, BotError};

use std::cmp::{Ordering, Ord};

pub trait Runnable {
	fn run(&self, user: User) -> BotResult<Option<String>>;
}

#[derive(Clone)]
pub struct Command {
    perms: CommandPerms,
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

impl Runnable for Command {
	fn run(&self, user: User) -> BotResult<Option<String>> {
        if self.perms > CommandPerms::max(&user) {
            return Err(BotError::Command(CommandError::InsufficientPerms));
        }
        self.runs += 1;
        self.action.run()
	}
}

pub struct Trigger {
	priority: usize,
	action: CommandAction,
	runs: usize
}

impl Trigger {
	pub fn new(priority: usize, action: CommandAction) -> Self {
		Self {
			priority,
			action,
			runs: 0
		}
	}
}

impl Runnable for Trigger {
	fn run(&self, _user: User) -> BotResult<Option<String>> {
        self.runs += 1;
        self.action.run()
	}
}

impl PartialEq for Trigger {
	fn eq(&self, other: &Self) -> bool {
		self.priority == other.priority
	}
}
impl Eq for Trigger {}
impl PartialOrd for Trigger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.priority.cmp(&other.priority))
	}
}
impl Ord for Trigger {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

