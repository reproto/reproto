#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RpName {
    pub prefix: Option<String>,
    pub parts: Vec<String>,
}

impl RpName {
    pub fn without_prefix(&self) -> RpName {
        RpName {
            prefix: None,
            parts: self.parts.clone(),
        }
    }

    pub fn with_parts(parts: Vec<String>) -> RpName {
        RpName {
            prefix: None,
            parts: parts,
        }
    }

    pub fn extend(&self, part: String) -> RpName {
        let mut parts = self.parts.clone();
        parts.push(part);

        RpName {
            prefix: self.prefix.clone(),
            parts: parts,
        }
    }
}

impl ::std::fmt::Display for RpName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Some(ref prefix) = self.prefix {
            write!(f, "{}::", prefix)?;
        }

        write!(f, "{}", self.parts.join("."))
    }
}
