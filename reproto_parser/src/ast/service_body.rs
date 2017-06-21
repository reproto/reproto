use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct ServiceBody<'a> {
    pub name: &'a str,
    pub comment: Vec<&'a str>,
    pub children: Vec<ServiceNested<'a>>,
}

enum Node {
    Endpoint {
        parent: Option<Rc<RefCell<Node>>>,
        url: RpLoc<String>,
        options: Vec<RpLoc<RpOptionDecl>>,
        #[allow(dead_code)]
        comment: Vec<String>,
        returns: Vec<RpServiceReturns>,
    },
    Star {
        parent: Option<Rc<RefCell<Node>>>,
        options: Vec<RpLoc<RpOptionDecl>>,
        #[allow(dead_code)]
        comment: Vec<String>,
        returns: Vec<RpServiceReturns>,
    },
}

impl Node {
    fn push_returns(&mut self, input: RpServiceReturns) {
        match *self {
            Node::Endpoint { url: _, parent: _, options: _, comment: _, ref mut returns } => {
                returns.push(input)
            }
            Node::Star { parent: _, options: _, comment: _, ref mut returns } => {
                returns.push(input)
            }
        }
    }
}

fn convert_return(path: &Path,
                  comment: Vec<String>,
                  ty: AstLoc<RpType>,
                  options: Vec<AstLoc<OptionDecl>>)
                  -> Result<RpServiceReturns> {
    let options = Options::new(options.into_model(path)?);

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
        ty: ty.into_model(path)?,
        produces: produces,
        status: status,
    })
}

fn unwind(parent: Option<Rc<RefCell<Node>>>) -> Result<RpServiceEndpoint> {
    use self::Node::*;

    let mut url: Vec<String> = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut returns = Vec::new();
    let mut comment = Vec::new();

    let mut current = parent;

    while let Some(step) = current {
        match *step.borrow() {
            Endpoint { parent: ref next_parent,
                       url: ref next_url,
                       options: ref next_options,
                       returns: ref next_returns,
                       comment: ref next_comment } => {
                current = next_parent.clone();
                url.push(next_url.as_ref().to_owned());
                options.extend(next_options.iter().map(Clone::clone).rev());
                returns.extend(next_returns.iter().map(Clone::clone).rev());
                comment.extend(next_comment.iter().map(Clone::clone).rev());
            }
            Star { parent: ref next_parent,
                   options: ref next_options,
                   returns: ref next_returns,
                   comment: ref next_comment } => {
                current = next_parent.clone();
                options.extend(next_options.iter().map(Clone::clone).rev());
                returns.extend(next_returns.iter().map(Clone::clone).rev());
                comment.extend(next_comment.iter().map(Clone::clone).rev());
            }
        }
    }

    let url: Vec<_> = url.into_iter().rev().collect();
    let url = url.join("");

    let options = Options::new(options.into_iter().rev().collect());

    let mut accepts: Vec<Mime> = Vec::new();

    for accept in options.find_all_strings("accept")? {
        let (value, pos) = accept.both();

        accepts.push(value.parse()
            .chain_err(|| ErrorKind::Pos("invalid mime type".to_owned(), pos.clone()))?);
    }

    let method: Option<String> = options.find_one_string("method")?
        .map(Loc::move_inner);

    let comment = comment.into_iter().rev().collect();
    let returns = returns.into_iter().rev().collect();

    Ok(RpServiceEndpoint {
        url: url,
        comment: comment,
        accepts: accepts,
        returns: returns,
        method: method,
    })
}

impl<'a> IntoModel for ServiceBody<'a> {
    type Output = Rc<RpServiceBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpServiceBody>> {
        let mut endpoints: Vec<RpServiceEndpoint> = Vec::new();

        let mut queue: Vec<(Option<Rc<RefCell<Node>>>, Vec<ServiceNested>)> = Vec::new();
        queue.push((None, self.children));

        while let Some((parent, children)) = queue.pop() {
            let is_terminus = children.iter().all(ServiceNested::is_returns);

            for child in children {
                match child {
                    // add to previous, including url changes.
                    ServiceNested::Endpoint { url, comment, options, children } => {
                        let node = Rc::new(RefCell::new(Node::Endpoint {
                            parent: parent.as_ref().map(Clone::clone),
                            url: url.into_model(path)?,
                            options: options.into_model(path)?,
                            comment: comment.into_iter().map(ToOwned::to_owned).collect(),
                            returns: Vec::new(),
                        }));

                        queue.push((Some(node.clone()), children));
                    }
                    // just add to previous without url changes.
                    ServiceNested::Star { comment, options, children } => {
                        let node = Rc::new(RefCell::new(Node::Star {
                            parent: parent.as_ref().map(Clone::clone),
                            options: options.into_model(path)?,
                            comment: comment.into_iter().map(ToOwned::to_owned).collect(),
                            returns: Vec::new(),
                        }));

                        queue.push((Some(node.clone()), children));
                    }
                    // end node, manifest an endpoint.
                    ServiceNested::Returns { comment, ty, options } => {
                        let comment = comment.into_iter().map(ToOwned::to_owned).collect();
                        let response = convert_return(path, comment, ty, options)?;

                        if let Some(parent) = parent.as_ref() {
                            parent.borrow_mut().push_returns(response);
                        }
                    }
                }
            }

            if is_terminus {
                endpoints.push(unwind(parent.clone())?);
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
