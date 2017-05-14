use backend::Backend;

pub struct FasterXmlBackend {
}

impl FasterXmlBackend {
    pub fn new() -> FasterXmlBackend {
        FasterXmlBackend {}
    }
}

impl Backend for FasterXmlBackend {}
