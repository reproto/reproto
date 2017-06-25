use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct ServiceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub children: Vec<ServiceNested<'input>>,
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    method: Option<RpLoc<String>>,
    path: Option<RpLoc<RpPathSpec>>,
    options: Vec<RpLoc<RpOptionDecl>>,
    comment: Vec<String>,
    returns: Vec<RpServiceReturns>,
    accepts: Vec<RpServiceAccepts>,
}

impl Node {
    fn push_returns(&mut self, input: RpServiceReturns) {
        self.returns.push(input);
    }

    fn push_accepts(&mut self, input: RpServiceAccepts) {
        self.accepts.push(input);
    }
}

fn convert_return(comment: Vec<String>,
                  ty: Option<RpLoc<RpType>>,
                  options: Vec<RpLoc<OptionDecl>>)
                  -> Result<RpServiceReturns> {
    let options = Options::new(options.into_model()?);

    let produces: Option<RpLoc<String>> = options.find_one_string("produces")?;

    let produces = if let Some(produces) = produces {
        let (produces, pos) = produces.both();

        let produces = produces.parse()
            .chain_err(|| ErrorKind::Pos("not a valid mime type".to_owned(), pos.into()))?;

        Some(produces)
    } else {
        None
    };

    let status: Option<RpLoc<RpNumber>> = options.find_one_number("status")?;

    let status = if let Some(status) = status {
        let (status, pos) = status.both();

        let status = status.to_u32()
            .ok_or_else(|| ErrorKind::Pos("not a valid status".to_owned(), pos.into()))?;

        Some(status)
    } else {
        None
    };

    Ok(RpServiceReturns {
        comment: comment,
        ty: ty.into_model()?,
        produces: produces,
        status: status,
    })
}

fn convert_accepts(comment: Vec<String>,
                   ty: RpLoc<RpType>,
                   options: Vec<RpLoc<OptionDecl>>)
                   -> Result<RpServiceAccepts> {
    let options = Options::new(options.into_model()?);

    let accepts: Option<RpLoc<String>> = options.find_one_string("accept")?;

    let accepts = if let Some(accepts) = accepts {
        let (accepts, pos) = accepts.both();

        let accepts = accepts.parse()
            .chain_err(|| ErrorKind::Pos("not a valid mime type".to_owned(), pos.into()))?;

        Some(accepts)
    } else {
        None
    };

    Ok(RpServiceAccepts {
        comment: comment,
        ty: ty.into_model()?,
        accepts: accepts,
    })
}

/// Recursively unwind all inherited information about the given node, and convert to a service
/// endpoint.
fn unwind(node: Rc<RefCell<Node>>) -> Result<RpServiceEndpoint> {
    let mut method: Option<RpLoc<String>> = None;
    let mut path = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut returns = Vec::new();
    let mut accepts = Vec::new();

    let comment = node.try_borrow()?.comment.clone();

    let mut current = Some(node);

    while let Some(step) = current {
        let next = step.try_borrow()?;

        // set method if not set
        method = method.or_else(|| next.method.clone());

        if let Some(ref next_url) = next.path {
            // correct order by extending in reverse
            path.extend(next_url.as_ref().segments.iter().rev().map(Clone::clone));
        }

        options.extend(next.options.iter().map(Clone::clone).rev());
        returns.extend(next.returns.iter().map(Clone::clone));
        accepts.extend(next.accepts.iter().map(Clone::clone));

        current = next.parent.clone();
    }

    let path = RpPathSpec { segments: path.into_iter().rev().collect() };

    let _options = Options::new(options.into_iter().rev().collect());

    Ok(RpServiceEndpoint {
        method: method,
        path: path,
        comment: comment,
        returns: returns,
        accepts: accepts,
    })
}

impl<'input> IntoModel for ServiceBody<'input> {
    type Output = Rc<RpServiceBody>;

    fn into_model(self) -> Result<Rc<RpServiceBody>> {
        let mut endpoints: Vec<RpServiceEndpoint> = Vec::new();

        // collecting root declarations
        let root = Rc::new(RefCell::new(Node {
            parent: None,
            method: None,
            path: None,
            options: Vec::new(),
            comment: Vec::new(),
            returns: Vec::new(),
            accepts: Vec::new(),
        }));

        let mut queue = Vec::new();
        queue.push((root, self.children));

        while let Some((parent, children)) = queue.pop() {
            for child in children {
                match child {
                    ServiceNested::Endpoint { method, path, comment, options, children } => {
                        let node = Rc::new(RefCell::new(Node {
                            parent: Some(parent.clone()),
                            method: method.into_model()?,
                            path: path.into_model()?,
                            options: options.into_model()?,
                            comment: comment.into_iter().map(ToOwned::to_owned).collect(),
                            returns: Vec::new(),
                            accepts: Vec::new(),
                        }));

                        queue.push((node, children));
                    }
                    // end node, manifest an endpoint.
                    ServiceNested::Returns { comment, ty, options } => {
                        let comment = comment.into_iter().map(ToOwned::to_owned).collect();
                        let returns = convert_return(comment, ty, options)?;
                        parent.try_borrow_mut()?.push_returns(returns);
                    }
                    ServiceNested::Accepts { comment, ty, options } => {
                        let comment = comment.into_iter().map(ToOwned::to_owned).collect();
                        let accepts = convert_accepts(comment, ty, options)?;
                        parent.try_borrow_mut()?.push_accepts(accepts);
                    }
                }
            }

            let p = parent.as_ref().try_borrow()?;

            if p.method.is_some() {
                endpoints.push(unwind(parent.clone())?);
            }
        }

        let endpoints = endpoints.into_iter().rev().collect();

        let service_body = RpServiceBody {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            endpoints: endpoints,
        };

        Ok(Rc::new(service_body))
    }
}
