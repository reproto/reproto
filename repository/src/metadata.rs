use reproto_core::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    pub version: Version,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub deployments: Vec<Deployment>,
}
