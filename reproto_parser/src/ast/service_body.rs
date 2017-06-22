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
    url: Option<RpLoc<String>>,
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

    fn comment(&self) -> &Vec<String> {
        &self.comment
    }
}

fn convert_return(comment: Vec<String>,
                  ty: AstLoc<RpType>,
                  options: Vec<AstLoc<OptionDecl>>)
                  -> Result<RpServiceReturns> {
    let options = Options::new(options.into_model()?);

    let produces: Option<RpLoc<String>> = options.find_one_string("produces")?;

    let produces = if let Some(produces) = produces {
        let (produces, pos) = produces.both();

        let produces = produces.parse()
            .chain_err(|| ErrorKind::Pos("not a valid mime type".to_owned(), pos.clone()))?;

        Some(produces)
    } else {
        None
    };

    let status: Option<RpLoc<RpNumber>> = options.find_one_number("status")?;

    let status = if let Some(status) = status {
        let (status, pos) = status.both();

        let status = status.to_u32()
            .ok_or_else(|| ErrorKind::Pos("not a valid status".to_owned(), pos.clone()))?;

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
                   ty: AstLoc<RpType>,
                   options: Vec<AstLoc<OptionDecl>>)
                   -> Result<RpServiceAccepts> {
    let options = Options::new(options.into_model()?);

    let accepts: Option<RpLoc<String>> = options.find_one_string("accept")?;

    let accepts = if let Some(accepts) = accepts {
        let (accepts, pos) = accepts.both();

        let accepts = accepts.parse()
            .chain_err(|| ErrorKind::Pos("not a valid mime type".to_owned(), pos.clone()))?;

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
fn unwind(node: Option<Rc<RefCell<Node>>>, comment: Vec<String>) -> Result<RpServiceEndpoint> {
    let mut url: Vec<String> = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut returns = Vec::new();
    let mut accepts = Vec::new();

    let mut current = node;

    while let Some(step) = current {
        let next = step.borrow();
        current = next.parent.clone();

        if let Some(ref next_url) = next.url {
            url.push(next_url.as_ref().to_owned());
        }

        options.extend(next.options.iter().map(Clone::clone).rev());
        returns.extend(next.returns.iter().map(Clone::clone));
        accepts.extend(next.accepts.iter().map(Clone::clone));
    }

    let url: Vec<_> = url.into_iter().rev().collect();
    let url = url.join("");

    let options = Options::new(options.into_iter().rev().collect());

    let method: Option<String> = options.find_one_string("method")?
        .map(Loc::move_inner);

    Ok(RpServiceEndpoint {
        url: url,
        comment: comment,
        returns: returns,
        accepts: accepts,
        method: method,
    })
}

impl<'input> IntoModel for ServiceBody<'input> {
    type Output = Rc<RpServiceBody>;

    fn into_model(self) -> Result<Rc<RpServiceBody>> {
        let mut endpoints: Vec<RpServiceEndpoint> = Vec::new();

        let mut queue: Vec<(Option<Rc<RefCell<Node>>>, Vec<ServiceNested>)> = Vec::new();
        queue.push((None, self.children));

        while let Some((parent, children)) = queue.pop() {
            let is_terminus = children.iter().all(ServiceNested::is_terminus);

            for child in children {
                match child {
                    // add to previous, including url changes.
                    ServiceNested::Endpoint { url, comment, options, children } => {
                        let node = Rc::new(RefCell::new(Node {
                            parent: parent.as_ref().map(Clone::clone),
                            url: Some(url.into_model()?),
                            options: options.into_model()?,
                            comment: comment.into_iter().map(ToOwned::to_owned).collect(),
                            returns: Vec::new(),
                            accepts: Vec::new(),
                        }));

                        queue.push((Some(node.clone()), children));
                    }
                    // just add to previous without url changes.
                    ServiceNested::Star { comment, options, children } => {
                        let node = Rc::new(RefCell::new(Node {
                            parent: parent.as_ref().map(Clone::clone),
                            url: None,
                            options: options.into_model()?,
                            comment: comment.into_iter().map(ToOwned::to_owned).collect(),
                            returns: Vec::new(),
                            accepts: Vec::new(),
                        }));

                        queue.push((Some(node.clone()), children));
                    }
                    // end node, manifest an endpoint.
                    ServiceNested::Returns { comment, ty, options } => {
                        let comment = comment.into_iter().map(ToOwned::to_owned).collect();
                        let returns = convert_return(comment, ty, options)?;

                        if let Some(parent) = parent.as_ref() {
                            parent.try_borrow_mut()?.push_returns(returns);
                        }
                    }
                    ServiceNested::Accepts { comment, ty, options } => {
                        let comment = comment.into_iter().map(ToOwned::to_owned).collect();
                        let accepts = convert_accepts(comment, ty, options)?;

                        if let Some(parent) = parent.as_ref() {
                            parent.try_borrow_mut()?.push_accepts(accepts);
                        }
                    }
                }
            }

            if is_terminus {
                let comment = if let Some(ref parent) = parent {
                    parent.try_borrow()?.comment().clone()
                } else {
                    Vec::new()
                };

                endpoints.push(unwind(parent.clone(), comment)?);
                continue;
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
