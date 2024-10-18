use std::collections::HashMap;
use std::io::{Result as IOResult, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Response<'a> {
    protocol_version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self {
            protocol_version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<Response<'a>> for String {
    fn from(r: Response) -> Self {
        let res = r.clone();

        // response line
        // header lines
        // empty line
        // response body line
        let res_str = format!(
            "{} {} {}\r\nContent-Length:{}\r\n{}\r\n{}",
            &res.protocol_version(),
            &res.status_code(),
            &res.status_text(),
            &r.body().len(),
            &res.headers(),
            &res.body(),
        );

        println!("res_str:{:?}\r\n---------------", res_str);

        res_str
    }
}

impl<'a> Response<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Response<'a> {
        let mut response: Response<'a> = Response::default();

        if status_code != "200" {
            response.status_code = status_code;
        }

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut header_items = HashMap::new();
                header_items.insert("Content-Type", "text/plain");
                Some(header_items)
            }
        };

        response.body = body;

        response
    }

    pub fn protocol_version(&self) -> &str {
        self.protocol_version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let headers_map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut headers_str = String::new();

        for (k, v) in headers_map.iter() {
            headers_str.push_str(k);
            headers_str.push(':');
            headers_str.push_str(v);
            headers_str.push_str("\r\n");
        }

        headers_str
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(s) => s.as_str(),
            None => "",
        }
    }

    pub fn send<T: Write>(&self, stream: &mut T) -> IOResult<()> {
        let res = self.clone();

        let res_string: String = res.into();
        Write::write_all(stream, res_string.as_bytes())?;

        Ok(())
    }
}
