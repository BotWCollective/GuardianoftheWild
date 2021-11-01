use crate::user::User;
use super::CommandError;
use crate::{BotResult, BotError};

use std::cmp::Ordering;
use std::time::{Instant, Duration};
use std::str::FromStr;

#[derive(Clone)]
pub struct Command {
	cooldown: (Instant, Duration),
	perms: CommandPerms,
	case: bool,
    trigger: Option<TriggerInfo>,
    action: CommandAction,
    runs: usize,
}

impl Command {
    pub fn run(&mut self, user: User) -> BotResult<Option<String>> {
        if self.perms > CommandPerms::max(&user) {
            return Err(BotError::Command(CommandError::InsufficientPerms));
        }
        self.runs += 1;
        self.action.run()
    }
    pub fn case_sensitive(&self) -> bool {
		self.case
    }
    pub fn trigger(&self) -> bool {
		self.trigger.is_some()
    }
}

impl FromStr for Command {
	type Err = std::convert::Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut r = Self {
			cooldown: (Instant::now() - Duration::from_millis(10000000), Duration::ZERO),
			case: true,
			trigger: None,
			perms: CommandPerms::Anyone,
			action: CommandAction::Static{ret: "".into()},
			runs: 0
		};
		let mut action = if s.contains("${") {1} else {2};
		let mut split = s.split(' ');
		while let Some(w) = split.next() {
			if !w.starts_with('-') {
				break;
			}
			match w.to_ascii_lowercase().as_str() {
				"cs=true" => r.case = true,
				"cs=false" => r.case = false,
				"p=mod" => r.perms = CommandPerms::Mod,
				"p=anyone" => r.perms = CommandPerms::Anyone,
				"p=vip" => r.perms = CommandPerms::Vip,
				"p=broadcaster" => r.perms = CommandPerms::Broadcaster,
				"t=static" => action = 0,
				"t=sub" => action = 1,
				"t=js" => action = 2,
				s => {
					if s.starts_with("cd=") {
						if let Some((_, b)) = s.split_once('=') {
							if let Ok(n) = b.parse::<u64>() {
								r.cooldown.1 = Duration::from_millis(n);
							}
						}
					} else if s.starts_with("k=") {
						if let Some((_, b)) = s.split_once('=') {
							if let Ok(n) = b.parse::<usize>() {
								r.trigger = Some(TriggerInfo {priority: n});
							}
						}
					}
				}
			}
		}
		let command_name = split.next().unwrap_or("no");
		let resp = split.collect();
		if action == 0 {
			r.action = CommandAction::Static {ret: resp};
		} else if action == 1 {
			r.action = CommandAction::Template {template: resp};
		} else {
			r.action = CommandAction::Script {command: "node".into(), file: format!("./commands/{}.js", command_name)}
		}
		Ok(r)
	}
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

