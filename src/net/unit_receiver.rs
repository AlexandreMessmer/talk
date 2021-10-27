use std::mem;

use tokio::{
    io,
    io::{AsyncReadExt, ReadHalf},
};

use crate::net::Socket;

pub(in crate::net) struct UnitReceiver {
    read_half: ReadHalf<Box<dyn Socket>>,
    buffer: Vec<u8>,
}

impl UnitReceiver {
    pub fn new(read_half: ReadHalf<Box<dyn Socket>>) -> Self {
        UnitReceiver {
            read_half,
            buffer: Vec::new(),
        }
    }

    pub fn read_half(&self) -> &ReadHalf<Box<dyn Socket>> {
        &self.read_half
    }

    pub async fn receive(&mut self) -> io::Result<&mut Vec<u8>> {
        let mut size_buffer = [0; mem::size_of::<u32>()];
        self.read_half.read_exact(&mut size_buffer[..]).await?;
        let size = u32::from_le_bytes(size_buffer) as usize;

        self.buffer.resize(size, 0u8);
        self.read_half.read_exact(&mut self.buffer[..]).await?;

        Ok(&mut self.buffer)
    }
}
