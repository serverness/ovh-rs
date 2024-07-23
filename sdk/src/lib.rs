use thiserror::Error;
mod generated_sdk;

pub mod config;
pub mod context;

pub use generated_sdk::*;

#[derive(Error, Debug)]
/// Errors for interfaces related to configuration and authentication
pub enum OxideError {
    #[error(
    r"$OVH_HOST is set, but {0} has no corresponding token.\n
      Set $OVH_TOKEN."
    )]
    MissingToken(String),
    #[error("no authenticated hosts")]
    NoAuthenticatedHosts,
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
}