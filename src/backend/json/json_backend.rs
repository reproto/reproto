use super::*;

pub struct JsonBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
}

impl JsonBackend {
    pub fn new(_options: JsonOptions, env: Environment, listeners: Box<Listeners>) -> JsonBackend {
        JsonBackend {
            env: env,
            listeners: listeners,
        }
    }

    pub fn package_file(&self, package: &RpPackage) -> String {
        package.parts.join("_")
    }
}

impl PackageUtils for JsonBackend {}

impl Backend for JsonBackend {
    fn compiler<'a>(&'a self, options: CompilerOptions) -> Result<Box<Compiler<'a> + 'a>> {
        Ok(Box::new(JsonCompiler {
            out_path: options.out_path,
            processor: self,
        }))
    }

    fn verify(&self) -> Result<Vec<Error>> {
        Ok(vec![])
    }
}
