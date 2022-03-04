use super::CommandError;
use crate::format::{format, FormatArgs};
use crate::User;
use crate::{BotError, BotResult};
use crate::bot::Message;

use log::debug;
use std::cmp::Ordering;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Command {
    timer: Instant,
    cooldown: Duration,
    perms: CommandPerms,
    case: bool,
    trigger: Option<TriggerInfo>,
    action: CommandAction,
    runs: usize,
}

impl Command {
    pub fn run(&mut self, message: Message) -> BotResult<Option<String>> {
        self.runs += 1;
        self.timer = Instant::now();
        if self.on_cooldown() {
            Ok(None)
        } else {
            self.action.run(FormatArgs::new(self.runs, message.sender.username(), message.words))
        }
    }
    fn on_cooldown(&self) -> bool {
        self.timer + self.cooldown > Instant::now()
    }
    pub fn is_trigger(&self) -> bool {
        self.trigger.is_some()
    }
    pub fn perms(&self) -> CommandPerms {
        self.perms
    }
}

pub struct NewCommand {
    pub name: String,
    pub cmd: Command
}

impl FromStr for NewCommand {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Command {
            timer: Instant::now() - Duration::from_millis(100000),
            cooldown: Duration::ZERO,
            case: true,
            trigger: None,
            perms: CommandPerms::Anyone,
            action: CommandAction::Static { ret: "".into() },
            runs: 0,
        };
        let mut action = if s.contains("${") { 1 } else { 0 };
        let mut split = s.split(' ').skip(2);
        let mut command_name = "".into();
        while let Some(w) = split.next() {
            if !w.starts_with('-') {
                command_name = w;
                break;
            }
            match w.to_ascii_lowercase().as_str() {
                "-cs=true" => r.case = true,
                "-cs=false" => r.case = false,
                "-p=mod" => r.perms = CommandPerms::Mod,
                "-p=anyone" => r.perms = CommandPerms::Anyone,
                "-p=vip" => r.perms = CommandPerms::Vip,
                "-p=broadcaster" => r.perms = CommandPerms::Broadcaster,
                "-t=static" => action = 0,
                "-t=sub" => action = 1,
                "-t=js" => action = 2,
                s => {
                    if s.starts_with("-cd=") {
                        if let Some((_, b)) = s.split_once('=') {
                            if let Ok(n) = b.parse::<u64>() {
                                r.cooldown = Duration::from_millis(n);
                            }
                        }
                    } else if s.starts_with("-k=") {
                        if let Some((_, b)) = s.split_once('=') {
                            if let Ok(n) = b.parse::<usize>() {
                                debug!("setting trigger priority to {}", n);
                                r.trigger = Some(TriggerInfo { priority: n });
                            }
                        }
                    }
                }
            }
        }
        let resp = split.fold(String::new(), |mut a, b| {
            a.push_str(b);
            a.push(' ');
            a
        });
        if action == 0 {
            r.action = CommandAction::Static { ret: resp };
        } else if action == 1 {
            r.action = CommandAction::Template { template: resp };
        } else {
            r.action = CommandAction::Script {
                command: "node".into(),
                file: format!("./commands/{}.js", command_name),
            }
        }
        Ok(NewCommand {name: command_name.to_string(), cmd: r})
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
pub enum CommandPerms {
    Anyone,
    Vip,
    Mod,
    Broadcaster,
}


impl CommandPerms {
    pub fn max(user: &User) -> Self {
    use CommandPerms::*;
        if user
            .tag("badges")
            .map(|b| b.contains("broadcaster"))
            .unwrap_or(false)
        {
            Broadcaster
        } else if user.tag("mod").map(|m| m == "1").unwrap_or(false) {
            Mod
        } else if user
            .tag("badges")
            .map(|b| b.contains("vip"))
            .unwrap_or(false)
        {
            Vip
        } else {
            Anyone
        }
    }
}

#[derive(Clone, Debug)]
pub enum CommandAction {
    Static { ret: String },
    Template { template: String },
    Script { command: String, file: String },
}

impl CommandAction {
    fn run(&self, args: FormatArgs) -> BotResult<Option<String>> {
        match &self {
            CommandAction::Static { ret } => Ok(Some(ret.into())),
            CommandAction::Template { template } => format(template, args).map(|r| Some(r)),
            // do something eventually
            _ => Ok(Some("".into())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TriggerInfo {
    priority: usize,
    // probably more here later
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
