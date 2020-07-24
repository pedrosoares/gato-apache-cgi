use gato_core::kernel::HttpCoreHandler;
use gato_core::kernel::Provider;
use crate::ApacheHttpCore;

pub struct ApacheServiceProvider { }

impl ApacheServiceProvider {
    pub fn new() -> Box<Self> {
        return Box::new(ApacheServiceProvider {  });
    }
}

impl Provider for ApacheServiceProvider {
    fn boot(&self) {
        let apache_http_core = ApacheHttpCore::new();
        HttpCoreHandler::set_driver(Box::new(apache_http_core));
    }
}