use env_logger::{Env, Builder};
use gotw::{Bot, BotConfig};
use log::error;
use std::process::exit;
use std::env;

fn main() {
    let env = Env::default().filter("GOTW_LOG");
    Builder::from_env(env).format(|buf, record| {
		writeln!(buf, "[{}  {}] ({}): {}", buf.timestamp(), record.level(), record.module_path().unwrap_or(""), record.args())
    }).init();
    let cfg = BotConfig {
        username: env::var("GOTW_NICK").unwrap_or_else(|_| {error!("Environment variable GOTW_NICK not found."); exit(1)}),
        password: env::var("GOTW_PASS").unwrap_or_else(|_| {error!("Environment variable GOTW_PASS not found."); exit(1)}),
        channel: env::var("GOTW_CHAN").unwrap_or_else(|_| {error!("Environment variable GOTW_CHAN not found."); exit(1)}),
        prefix: env::var("GOTW_PREF").unwrap_or_else(|_| {error!("Environment variable GOTW_PREF not found."); exit(1)}),
    };
    let mut bot = Bot::login(cfg).unwrap_or_else(|e| {error!("{:?}", e); exit(1)});
    loop {
	    bot.try_parse_message().unwrap_or_else(|e| {error!("{:?}", e); None});
    }
}
