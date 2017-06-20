use naming;

pub struct Options {
    pub package_prefix: Option<String>,
    pub id_converter: Option<Box<naming::Naming>>,
    pub modules: Vec<String>,
}
