mod command_map;
mod commands;
pub use command_map::CommandMap;

#[derive(Debug)]
pub enum CommandError {
	NotFound,
	AlreadyRegistered,
	CommandFailed,
	NotEnoughArgs
}
