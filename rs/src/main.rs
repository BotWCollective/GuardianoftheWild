use env_logger::Env;
use gotw::{Bot, BotConfig};
use log::info;

fn main() {
    let env = Env::default();
    env_logger::init_from_env(env);
    let cfg = BotConfig {
        username: "guardianofthewild".into(),
        password: "oauth:1234abcd".into(),
        channel: "#botwcollective".into(),
        prefix: "!".into(),
    };
    let mut bot = Bot::login(cfg).unwrap();
    loop {
        info!("{:?}", bot.try_parse_message());
    }
}
