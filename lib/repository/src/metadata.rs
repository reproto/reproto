//! Deployment metadata

use crate::checksum::Checksum;
use core::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    pub version: Version,
    pub object: Checksum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub deployments: Vec<Deployment>,
}
