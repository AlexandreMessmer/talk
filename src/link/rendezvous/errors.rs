use crate::{
    link::rendezvous::ShardId, net::errors::PlainConnectionError,
    sync::fuse::errors::FuseError,
};

use snafu::Snafu;

use tokio::io::Error as TokioIoError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ServerError {
    #[snafu(display("`server` failed to initialize: {}", source))]
    InitializeFailed { source: TokioIoError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ClientError {
    #[snafu(display("card is already published (shard: {:?})", shard))]
    AlreadyPublished { shard: Option<ShardId> },
    #[snafu(display("shard id is invalid"))]
    ShardIdInvalid,
    #[snafu(display("shard is full"))]
    ShardFull,
    #[snafu(display("shard is incomplete"))]
    ShardIncomplete,
    #[snafu(display("card unknown"))]
    CardUnknown,
    #[snafu(display("address unknown"))]
    AddressUnknown,
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ListenError {
    #[snafu(display("`listen` interrupted: {}", source))]
    ListenInterrupted { source: FuseError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ServeError {
    #[snafu(display("`serve` interrupted: {}", source))]
    ServeInterrupted { source: FuseError },
    #[snafu(display("connection error: {}", source))]
    ConnectionError { source: PlainConnectionError },
}
