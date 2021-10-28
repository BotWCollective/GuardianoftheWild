use crate::user::User;

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

pub enum CommandResult {
    Success(String),
    InsufficientPermissions,
}

use CommandResult::*;

impl Command {
    pub fn new(perms: CommandPerms, action: CommandAction) -> Self {
        Self {
            perms,
            action,
            runs: 0,
        }
    }
    pub fn run(&mut self, user: User) -> CommandResult {
        if self.perms > CommandPerms::max(&user) {
            return InsufficientPermissions;
        }
        self.runs = self.runs + 1;
        match &self.action {
            CommandAction::Static { ret } => Success(ret.into()),
            // do something eventually
            _ => Success("".into()),
        }
    }
}
