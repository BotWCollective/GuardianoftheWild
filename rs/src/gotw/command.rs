mod command_map;
mod commands;
pub use command_map::CommandMap;

#[derive(Debug)]
pub enum CommandError {
    // for when a mod tries to alias/delete/show a command that doesn't exist
    NotRegistered,
    AlreadyRegistered,
    CommandFailed,
    NotEnoughArgs,
    InsufficientPerms
}
