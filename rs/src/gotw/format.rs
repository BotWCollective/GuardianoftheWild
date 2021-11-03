use crate::{BotResult, BotError, command::CommandError};

pub fn format(spec: &str, args: FormatArgs) -> BotResult<String> {
	let mut ret = String::new();
	let mut chars = spec.chars().peekable();
	while let Some(c) = chars.next() {
		if c == '$' {
			if let Some('{') = chars.peek() {
				chars.next();
				let mut var = String::new();
				while let Some(c) = chars.next() {
					if c != '}' {
						var.push(c);
					} else {
						break;
					}
				}
				if var == "count" {
					ret.push_str(&args.count.to_string());
				} else if var == "sender" {
					ret.push_str(&args.sender.to_string());
				} else if var.parse::<usize>().is_ok() {
					if let Some(v) = args.command_args.get(var.parse::<usize>().unwrap()) {
						ret.push_str(v);
					} else {
						return Err(BotError::Command(CommandError::NotEnoughArgs));
					}
				} else {
					ret.push_str("${");
					ret.push_str(&var);
					ret.push('}');
				}
			}
		} else {
			ret.push(c);
		}
	}
	Ok(ret)
}

pub struct FormatArgs {
	count: usize,
	sender: String,
	command_args: Vec<String>
}

impl FormatArgs {
	pub fn new(count: usize, sender: &str, command_args: Vec<String>) -> Self {
		Self {
			count, sender: sender.into(), command_args
		}
	}
}
