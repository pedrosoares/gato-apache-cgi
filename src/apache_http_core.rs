use std::env;
use std::collections::HashMap;
use gato_core::kernel::{HttpCore, RouterHandler, Request, Logger};

pub struct ApacheHttpCore { }

impl ApacheHttpCore {
    pub fn new() -> ApacheHttpCore {
        return ApacheHttpCore { };
    }
}

impl HttpCore for ApacheHttpCore {

    fn handle(&self) {
        Logger::info("ApacheHttpCore[handle]");
        // Initialize the Variable Method
        let method = env::var("REQUEST_METHOD").unwrap_or("GET".to_string());
        // Initialize the Variable Method
        let uri = env::var("REQUEST_URI").unwrap_or("/".to_string());
        // Create Request
        let mut request = Request::new(method, uri, self.get_request_headers());
        // Get RouterHandler Driver
        let router_handler = RouterHandler::get_driver();
        // Get Response from RouterHandle
        let response = router_handler.handle(&mut request);
        // Get Response Headers
        let response_headers = response.get_headers();
        // Print Apache Status Code
        print!("Status: {}\n", response.get_code());
        // Send to Apache the HEADERS
        for header in response_headers {
            print!("{}: {}\n", header.0, header.1);
        }
        // Send to Apache the BODY
        print!("\n{}", response.get_body());
    }

    fn get_request_headers(&self) -> HashMap<String, String> {
        let mut request_headers: HashMap<String, String> = HashMap::new();
        let args: Vec<_> = env::args().collect();
        for arg in args {
            let pieces: Vec<&str> = arg.split("=").collect();
            if pieces.len() == 2 {
                request_headers.insert(pieces[0].to_string(), pieces[1].to_string());
            }
        }
        return request_headers;
    }

    fn get_post_data(&self) -> String {
        let mut post_tmp = "".to_string();
        // Get from std::cin the POST DATA as String
        loop {
            let mut line = String::new();
            let result = std::io::stdin().read_line(&mut line);
            if result.is_err() || line.is_empty() {
                break;
            }
            post_tmp += line.as_str();
        }
        return post_tmp;
    }
}