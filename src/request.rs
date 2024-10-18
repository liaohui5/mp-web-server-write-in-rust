use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "PATCH" => Method::Patch,
            "DELETE" => Method::Delete,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ProtocolVersion {
    HTTP1_1,
    HTTP2,
    Uninitialized,
}
impl From<&str> for ProtocolVersion {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => ProtocolVersion::HTTP1_1,
            "HTTP/2" => ProtocolVersion::HTTP2,
            _ => ProtocolVersion::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub protocol_version: ProtocolVersion,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}
impl From<String> for Request {
    fn from(req_str: String) -> Self {
        let mut method = Method::Uninitialized;
        let mut protocol_version = ProtocolVersion::Uninitialized;
        let mut resource = Resource::Path(String::new());
        let mut headers = HashMap::new();
        let mut request_body = "";

        let iter = req_str.lines();

        for line in iter {
            if line.contains("HTTP") {
                let (m, r, v) = process_req_line(line);
                method = m;
                resource = r;
                protocol_version = v;
            } else if line.contains(":") {
                let (key, val) = process_header(line);
                headers.insert(key, val);
            } else if line.is_empty() {
                // empty line
            } else {
                request_body = line;
            }
        }

        Request {
            method,
            protocol_version,
            headers,
            resource,
            body: request_body.into(),
        }
    }
}

// parse request line
fn process_req_line(line: &str) -> (Method, Resource, ProtocolVersion) {
    let mut words = line.split_whitespace(); // split by space charetacter
    let m = words.next().unwrap();
    let r = words.next().unwrap();
    let v = words.next().unwrap();
    (m.into(), Resource::Path(String::from(r)), v.into())
}

// parse request headers
fn process_header(line: &str) -> (String, String) {
    let mut key = String::new();
    let mut val = String::new();
    let mut iter = line.split(":");

    if let Some(k) = iter.next() {
        key = k.trim().to_string();
    }

    if let Some(v) = iter.next() {
        val = v.trim().to_string();
    }

    (key, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_method_get_variant() {
        assert_eq!(Method::Get, Method::from("GET"));

        // after implements From trait, can be use `into` function
        assert_eq!(Method::Get, "GET".into());
    }

    #[test]
    fn should_return_protocol_version_variant() {
        assert_eq!(ProtocolVersion::HTTP1_1, "HTTP/1.1".into());
    }

    #[test]
    fn should_parse_req_line_string_to_enums() {
        let (m, r, v) = process_req_line("GET /test.html HTTP/1.1");
        assert_eq!(m, Method::Get);
        assert_eq!(r, Resource::Path("/test.html".to_string()));
        assert_eq!(v, ProtocolVersion::HTTP1_1);
    }

    #[test]
    fn should_parse_header_line_string_to_tuple() {
        let (k, v) = process_header("Accept:text/html");
        assert_eq!(k, String::from("Accept"));
        assert_eq!(v, String::from("text/html"));

        // trim spaces
        let (k, v) = process_header(" Host : localhost ");
        assert_eq!(k, String::from("Host"));
        assert_eq!(v, String::from("localhost"));
    }

    #[test]
    fn should_parse_string_to_request_struct() {
        // parse str to Request struct
        let req_str =
            String::from("GET /search HTTP/1.1\r\nAccept:text/html\r\nHost:localhost\r\n\r\nhello");
        let request = Request::from(req_str);

        // expected request instance
        let expected = Request {
            method: Method::Get,
            protocol_version: ProtocolVersion::HTTP1_1,
            resource: Resource::Path(String::from("/search")),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Accept".to_string(), "text/html".to_string());
                headers.insert("Host".to_string(), "localhost".to_string());
                headers
            },
            body: String::from("hello"),
        };

        assert_eq!(expected.method, request.method);
        assert_eq!(expected.protocol_version, request.protocol_version);
        assert_eq!(expected.resource, request.resource);
        assert_eq!(expected.headers, request.headers);
    }
}
