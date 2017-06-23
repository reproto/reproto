use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceEndpoint {
    pub method: Option<RpLoc<String>>,
    pub path: RpPathSpec,
    pub comment: Vec<String>,
    pub accepts: Vec<RpServiceAccepts>,
    pub returns: Vec<RpServiceReturns>,
}

impl RpServiceEndpoint {
    pub fn url(&self) -> String {
        self.path.url()
    }

    pub fn id_parts<F>(&self, filter: F) -> Vec<String>
        where F: Fn(&str) -> String
    {
        let mut parts = Vec::new();

        if let Some(ref method) = self.method {
            parts.push(filter(method.as_ref().as_ref()));
        }

        parts.extend(self.path.id_fragments().into_iter().map(filter));
        parts
    }

    pub fn method(&self) -> Option<&str> {
        self.method.as_ref().map(|v| v.as_ref().as_ref())
    }
}
