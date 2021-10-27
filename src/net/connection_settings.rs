use std::time::Duration;

use crate::net::{ReceiverSettings, SenderSettings};

#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    send_timeout: Option<Duration>,
    receive_timeout: Option<Duration>,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        ConnectionSettings {
            send_timeout: None,
            receive_timeout: None,
        }
    }
}

impl ConnectionSettings {
    pub fn split(self) -> (SenderSettings, ReceiverSettings) {
        (
            SenderSettings {
                send_timeout: self.send_timeout,
            },
            ReceiverSettings {
                receive_timeout: self.receive_timeout,
            },
        )
    }

    pub fn send_timeout(&self) -> &Option<Duration> {
        &self.send_timeout
    }

    pub fn receive_timeout(&self) -> &Option<Duration> {
        &self.receive_timeout
    }
}
