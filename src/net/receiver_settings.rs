use std::time::Duration;

use crate::net::ConnectionSettings;

#[derive(Debug, Clone)]
pub struct ReceiverSettings {
    pub receive_timeout: Option<Duration>,
}

impl Default for ReceiverSettings {
    fn default() -> Self {
        ReceiverSettings {
            receive_timeout: ConnectionSettings::default()
                .receive_timeout()
                .to_owned(),
        }
    }
}
