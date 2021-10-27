use doomstack::{here, Doom, ResultExt, Top};
use serde::Serialize;

use crate::{
    crypto::primitives::channel::Sender as ChannelSender,
    net::{SecureConnectionError, SenderSettings, UnitSender},
    time,
};

pub struct SecureSender {
    unit_sender: UnitSender,
    send_buffer: Vec<u8>,
    channel_sender: ChannelSender,
    settings: SenderSettings,
}

impl SecureSender {
    pub(in crate::net) fn new(
        unit_sender: UnitSender,
        channel_sender: ChannelSender,
        settings: SenderSettings,
    ) -> Self {
        Self {
            unit_sender,
            send_buffer: Vec::new(),
            channel_sender,
            settings,
        }
    }

    pub fn configure(&mut self, settings: SenderSettings) {
        self.settings = settings;
    }

    pub async fn send<M>(
        &mut self,
        message: &M,
    ) -> Result<(), Top<SecureConnectionError>>
    where
        M: Serialize,
    {
        self.send_buffer.clear();
        self.channel_sender
            .encrypt_into(message, &mut self.send_buffer)
            .pot(SecureConnectionError::EncryptFailed, here!())?;

        time::optional_timeout(
            self.settings.send_timeout,
            self.unit_sender.send(&self.send_buffer),
        )
        .await
        .pot(SecureConnectionError::SendTimeout, here!())?
        .map_err(SecureConnectionError::write_failed)
        .map_err(Doom::into_top)
        .spot(here!())
    }

    pub async fn send_plain<M>(
        &mut self,
        message: &M,
    ) -> Result<(), Top<SecureConnectionError>>
    where
        M: Serialize,
    {
        self.send_buffer.clear();
        self.channel_sender
            .authenticate_into(message, &mut self.send_buffer)
            .pot(SecureConnectionError::MacComputeFailed, here!())?;

        time::optional_timeout(
            self.settings.send_timeout,
            self.unit_sender.send(&self.send_buffer),
        )
        .await
        .pot(SecureConnectionError::SendTimeout, here!())?
        .map_err(SecureConnectionError::write_failed)
        .map_err(Doom::into_top)
        .spot(here!())
    }
}
