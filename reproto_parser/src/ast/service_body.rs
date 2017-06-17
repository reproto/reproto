use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct ServiceBody {
    pub name: String,
    pub comment: Vec<String>,
}

impl IntoModel for ServiceBody {
    type Output = Rc<RpServiceBody>;

    fn into_model(self, _pos: &RpPos) -> Result<Rc<RpServiceBody>> {
        let service_body = RpServiceBody {
            name: self.name,
            comment: self.comment,
        };

        Ok(Rc::new(service_body))
    }
}
