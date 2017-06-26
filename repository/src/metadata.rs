use reproto_core::Version;
use sha256::Checksum;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    pub version: Version,
    pub object: Checksum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub deployments: Vec<Deployment>,
}
