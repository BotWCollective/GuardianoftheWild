use env_logger::{fmt::Color, WriteStyle};
use env_logger::{Builder, Env};
use gotw::{Bot, BotConfig};
use log::{error, info, Level};
use std::io::Write;
use std::process::exit;

fn main() {
    let env = Env::default().filter("GOTW_LOG");
    Builder::from_env(env)
        .format(|buf, record| {
            let mut level_style = buf.style();
            level_style.set_bold(true);
            match record.level() {
                Level::Error => {
                    level_style.set_color(Color::Red);
                }
                Level::Warn => {
                    level_style.set_color(Color::Yellow);
                }
                Level::Info => {
                    level_style.set_color(Color::Green);
                }
                Level::Debug => {
                    level_style.set_color(Color::Blue);
                }
                Level::Trace => {
                    level_style.set_color(Color::Cyan);
                }
            }
            writeln!(
                buf,
                "[{} {}] ({}): {}",
                buf.timestamp(),
                level_style.value(record.level()),
                record.module_path().unwrap_or(""),
                record.args()
            )
        })
        .write_style(WriteStyle::Auto)
        .init();
    let cfg = BotConfig::from_env().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    });
    let mut bot = Bot::login(cfg).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    });
    bot.wait_commands().unwrap();
}
