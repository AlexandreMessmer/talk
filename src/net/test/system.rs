use crate::{
    crypto::{primitives::sign::PublicKey, KeyChain},
    net::{
        test::{ConnectionPair, TestConnector, TestListener},
        Connector, Listener,
    },
    time::test::join,
};

use futures::stream::{FuturesOrdered, StreamExt};

use std::collections::HashMap;
use std::net::SocketAddr;

pub struct System {
    pub keys: Vec<PublicKey>,
    pub connectors: Vec<TestConnector>,
    pub listeners: Vec<TestListener>,
}

impl System {
    pub fn new(
        keys: Vec<PublicKey>,
        connectors: Vec<TestConnector>,
        listeners: Vec<TestListener>,
    ) -> Self {
        System {
            keys,
            connectors,
            listeners,
        }
    }

    pub async fn setup(peers: usize) -> System {
        let keychains =
            (0..peers).map(|_| KeyChain::random()).collect::<Vec<_>>();

        let roots = keychains
            .iter()
            .map(|keychain| keychain.keycard().root())
            .collect::<Vec<_>>();

        let (listeners, addresses): (Vec<TestListener>, Vec<SocketAddr>) =
            keychains
                .iter()
                .map(|keychain| async move {
                    TestListener::new(keychain.clone()).await
                })
                .collect::<FuturesOrdered<_>>()
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .unzip();

        let peers: HashMap<PublicKey, SocketAddr> = roots
            .clone()
            .into_iter()
            .zip(addresses.into_iter())
            .collect();

        let connectors: Vec<TestConnector> = keychains
            .into_iter()
            .map(|keychain| TestConnector::new(keychain, peers.clone()))
            .collect();

        System::new(roots, connectors, listeners)
    }

    pub async fn connect(
        &mut self,
        source: usize,
        destination: usize,
    ) -> ConnectionPair {
        let source_future =
            self.connectors[source].connect(self.keys[destination]);
        let destination_future = self.listeners[destination].accept();

        let (source, destination) =
            futures::join!(source_future, destination_future);

        ConnectionPair::new(source.unwrap(), destination.unwrap().1)
    }

    pub async fn connection_matrix(&mut self) -> Vec<Vec<ConnectionPair>> {
        let mut matrix = Vec::with_capacity(self.keys.len());

        for sender in 0..self.keys.len() {
            let mut row = Vec::with_capacity(self.keys.len());

            for receiver in 0..self.keys.len() {
                row.push(self.connect(sender, receiver).await);
            }

            matrix.push(row);
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn example_setup() {
        let mut system = System::setup(8).await;

        let handles = system
            .connection_matrix()
            .await
            .into_iter()
            .map(|row| {
                row.into_iter().map(|mut pair| {
                    tokio::spawn(async move {
                        let sent: u32 = 42;
                        let received: u32 = pair.transmit(&sent).await.unwrap();

                        assert_eq!(received, sent);
                    })
                })
            })
            .flatten();

        join(handles).await.unwrap();
    }
}
