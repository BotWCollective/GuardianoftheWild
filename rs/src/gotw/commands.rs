use crate::user::User;
use std::cmp::Ordering;

pub struct Command {
	perms: CommandPerms,
	action: CommandAction,
	runs: usize
}


#[derive(PartialEq, Eq, PartialOrd)]
pub enum CommandPerms {
	Anyone,
	Vip,
	Mod,
	Broadcaster
}

use CommandPerms::*;

impl CommandPerms {
	pub fn max(user: &User) -> Self {
		if user.tag("badges").unwrap_or(&"".to_string()).contains("broadcaster") {
			Broadcaster
		} else if user.tag("mod").unwrap_or(&"".to_string()) == "1" {
			Mod
		} else if user.tag("badges").unwrap_or(&"".to_string()).contains("vip") {
			Vip
		} else {
			Anyone
		}
	}
}

impl Command {
	pub fn new(perms: CommandPerms, action: CommandAction) -> Self {
		Self {
			perms, action, runs: 0
		}
	}
	pub fn run(&mut self, user: User) -> bool {
		if self.perms > CommandPerms::max(&user) {
			return false;
		}
		// do something eventually
		self.runs = self.runs + 1;
		true
	}
}

pub enum CommandAction {
	Static {ret: String},
	Template {template: String},
	Script {command: String, file: String}
}

