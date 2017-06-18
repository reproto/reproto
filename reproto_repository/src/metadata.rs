use semver;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    pub versions: Vec<semver::Version>,
    #[serde(default)]
    snapshots: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_deserialize() {
        let metadata: Metadata = toml::from_str("versions = [\"0.0.1\", \"0.0.2\"]").unwrap();
        println!("metadata = {:?}", metadata);
    }
}
