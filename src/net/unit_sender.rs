use tokio::{
    io,
    io::{AsyncWriteExt, WriteHalf},
};

use crate::net::Socket;

pub(in crate::net) struct UnitSender {
    write_half: WriteHalf<Box<dyn Socket>>,
}

impl UnitSender {
    pub fn new(write_half: WriteHalf<Box<dyn Socket>>) -> Self {
        UnitSender { write_half }
    }

    pub fn write_half(&self) -> &WriteHalf<Box<dyn Socket>> {
        &self.write_half
    }

    pub async fn send(&mut self, msg: &Vec<u8>) -> io::Result<()> {
        let size = (msg.len() as u32).to_le_bytes();

        self.write_half.write_all(&size).await?;
        self.write_half.write_all(msg).await?;

        Ok(())
    }
}
