use crate::BotResult;
use log::{debug, info};
use std::net::TcpStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

pub struct TwitchIrcClient {
    conn: WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>,
    _nick: String,
    _pass: String,
    channel: String,
}

impl TwitchIrcClient {
    pub fn connect(nick: String, pass: String, channel: String) -> BotResult<Self> {
        let (sock, _) =
            connect(Url::parse("ws://irc-ws.chat.twitch.tv:80").unwrap()).expect("cant connect");
        let mut r = Self {
            conn: sock,
            _nick: nick.clone(),
            _pass: pass.clone(),
            channel: channel.clone(),
        };
        info!("Connected to Twitch");
        r.send_cmd("PASS", &pass)?;
        info!("Sent password");
        r.send_cmd("NICK", &nick)?;
        info!("Identified as {}", nick);
        r.send_cmd("JOIN", &channel)?;
        info!("Joined {}", channel);
        r.send_cmd("CAP REQ", ":twitch.tv/tags twitch.tv/commands")?;
        // throw out the twitch welcome messages
        r.get_message()?;
        r.get_message()?;
        r.get_message()?;
        Ok(r)
    }
    pub fn get_message(&mut self) -> BotResult<String> {
        let msg = self.conn.read_message()?;
        debug!("Got message from Twitch: {:?}", msg);
        if let Message::Text(t) = msg {
            if t == "PING :tmi.twitch.tv\r\n" {
                debug!("Got Ping");
                self.conn
                    .write_message(Message::Text("PONG :tmi.twitch.tv".into()))?;
                debug!("Sent Pong");
                return Ok("".into());
            }
            return Ok(t);
        }
        Ok("".into())
    }
    pub fn send_message(&mut self, message: &str) -> BotResult<()> {
        self.send_cmd(
            &format!("PRIVMSG {}", self.channel),
            &format!(":{}", message),
        )
    }
    pub fn send_cmd(&mut self, cmd: &str, message: &str) -> BotResult<()> {
        let r = self
            .conn
            .write_message(Message::Text(format!("{} {}", cmd, message)))
            .map_err(|e| e.into());
        debug!("Sent {} {}", cmd, message);
        r
    }
}
