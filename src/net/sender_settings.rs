use std::time::Duration;

use crate::net::ConnectionSettings;

#[derive(Debug, Clone)]
pub struct SenderSettings {
    pub send_timeout: Option<Duration>,
}

impl Default for SenderSettings {
    fn default() -> Self {
        SenderSettings {
            send_timeout: ConnectionSettings::default()
                .send_timeout()
                .to_owned(),
        }
    }
}
