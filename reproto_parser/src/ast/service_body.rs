use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct ServiceBody {
    pub name: String,
    pub comment: Vec<String>,
    pub children: Vec<ServiceNested>,
}

struct ResponseNode {
    parent: Rc<EndpointNode>,
}

struct EndpointNode {
    parent: Option<Rc<EndpointNode>>,
    endpoint: ServiceEndpoint,
}

impl IntoModel for ServiceBody {
    type Output = Rc<RpServiceBody>;

    fn into_model(self, _pos: &Path) -> Result<Rc<RpServiceBody>> {
        let mut queue = Vec::new();
        queue.push((None, self.children));

        for child in self.children {
            match child {
                ServiceNested::Endpoint { comment, options, children } => {
                }
                ServiceNested::Response { comment, ty, children } => {
                }
            }
        }

        let service_body = RpServiceBody {
            name: self.name,
            comment: self.comment,
            endpoints: vec![],
        }

        Ok(Rc::new(service_body))
    }
}
