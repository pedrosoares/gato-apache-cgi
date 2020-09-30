use std::env;
use std::collections::HashMap;
use gato_core::kernel::{HttpCore, RouterHandler, Request, Logger};

pub struct ApacheHttpCore { }

impl ApacheHttpCore {
    pub fn new() -> ApacheHttpCore {
        return ApacheHttpCore { };
    }

    fn first_letter_to_upper_case (&self, s1: String) -> String {
        let mut c = s1.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

impl HttpCore for ApacheHttpCore {

    fn handle(&self) {
        Logger::info("ApacheHttpCore[handle]");
        // Initialize the Variable Method
        let method = env::var("REQUEST_METHOD").unwrap_or("GET".to_string());
        // Initialize the Variable Method
        let uri = env::var("REQUEST_URI").unwrap_or("/".to_string());
        // TODO change request object to RequestBuilder
        // Create Request
        let mut request = Request::new(method, uri, self.get_request_headers());
        // Add request body if exits to request object
        request.add_body(self.get_post_data());
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
        let vars: Vec<_>  = env::vars().collect();
        for arg in vars {
            if arg.0.starts_with("HTTP_") || arg.0 == "CONTENT_TYPE"  || arg.0 == "CONTENT_LENGTH" {
                let mut name = vec![];
                let tmp = arg.0.replace("HTTP_", "");
                let names = tmp.split("_");
                for n in names {
                    name.push(self.first_letter_to_upper_case(n.to_lowercase()));
                }
                request_headers.insert(name.join("-"), arg.1);
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
