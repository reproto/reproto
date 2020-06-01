//! Declares features and what version they are introduced for.

use crate::core::errors::Error;
use crate::core::Version;
use std::collections::HashMap;

pub struct Features {
    features: HashMap<&'static str, Feature>,
}

impl Features {
    pub fn new() -> Result<Self, Error> {
        let mut features = HashMap::new();

        features.insert(
            "format_attribute",
            Feature {
                name: "format_attribute",
                stable_at: None,
            },
        );

        Ok(Features { features })
    }

    /// Access the given feature.
    pub fn get(&self, name: &str) -> Option<&Feature> {
        self.features.get(name)
    }
}

pub struct Feature {
    /// Name of the feature.
    pub name: &'static str,
    /// Version of the schema that the feature was stabilizied.
    pub stable_at: Option<Version>,
}
