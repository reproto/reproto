//! Deployment metadata

use checksum::Checksum;
use core::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    pub version: Version,
    pub object: Checksum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub deployments: Vec<Deployment>,
}
