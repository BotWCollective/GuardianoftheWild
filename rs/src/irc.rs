use tokio::net::TcpStream;
use std::io::Result as IoResult;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub struct IrcClient {
	conn: TcpStream,
	name: String,
	pass: String,
	channel: String,
	buf: [u8; 1024]
}

impl IrcClient {
	pub async fn connect(addr: (String, u16), name: String, pass: String, channel: String) -> IoResult<Self> {
		let conn = TcpStream::connect(addr).await?;
		let mut client = Self {conn, name: name.to_ascii_lowercase(), pass, channel: channel.to_ascii_lowercase(), buf: [0; 1024]};
		client.send("PASS", &client.pass.clone()).await?;
		client.send("NICK", &client.name.clone()).await?;
		client.send("JOIN", &client.channel.clone()).await?;
		Ok(client)
	}
	pub async fn get_message(&mut self) -> IoResult<String> {
		self.buf = [0; 1024];
		let bytes = self.conn.read(&mut self.buf).await?;
		let msg = unsafe {String::from_utf8_unchecked(self.buf[..bytes].to_vec())};
		if msg == "PING :tmi.twitch.tv" {
			self.pong().await?;
			return Ok("".into());
		}
		Ok(msg)
	}
	pub async fn send_message(&mut self, message: &str) -> IoResult<()> {
		self.send("PRIVMSG", message).await
	}
	async fn send(&mut self, command: &str, message: &str) -> IoResult<()> {
		self.conn.write(&(format!("{} {}", command, message).into_bytes())[..]).await.map(|_| ())
	}
	async fn pong(&mut self) -> IoResult<()> {
		self.conn.write(b"PONG :tmi.twitch.tv").await?;
		Ok(())
	}
}
