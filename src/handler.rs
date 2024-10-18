use crate::request;
use crate::{request::Request, response::Response};
use colored::Colorize;
use std::collections::HashMap;
use std::time::Duration;
use std::{env, fs, thread};

pub trait Handler {
    fn handle(req: &Request) -> Response;
    fn load_file(file_name: &str) -> Option<String> {
        // use the CARGO_MANIFEST_DIR constant to get the root path of the current package
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let file_path = format!("{}/{}", public_path, file_name);

        println!("{}", format!("file_path:{}", file_path).purple());

        let contents = fs::read_to_string(file_path);

        // convert Result to Option
        contents.ok()
    }
}

pub struct ApiHandler;
pub struct StaticFileHandler;
pub struct NotFoundHandler;

// Not Found
impl Handler for NotFoundHandler {
    fn handle(_: &Request) -> Response {
        Response::new("404", None, Self::load_file("404.html"))
    }
}

// static file
impl Handler for StaticFileHandler {
    fn handle(req: &Request) -> Response {
        let request::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        // default response header
        let mut headers: HashMap<&str, &str> = HashMap::new();
        headers.insert("Content-Type", "text/html");

        println!("{}:{:?}", "route".cyan(), route);
        match route[1] {
            "" | "index.html" => Response::new("200", Some(headers), Self::load_file("index.html")),
            "sleep.html" => {
                // for test multiple threads
                thread::sleep(Duration::from_secs(5));
                Response::new("200", Some(headers), Self::load_file("sleep.html"))
            }
            path => {
                if let Some(contents) = Self::load_file(path) {
                    println!("{}", format!("req_path:{}", path).cyan());

                    // set Content-Type for override response header
                    if path.ends_with(".css") {
                        headers.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        headers.insert("Content-Type", "text/javascript");
                    } else if path.ends_with(".html") {
                        headers.insert("Content-Type", "text/html");
                    } else {
                        headers.insert("Content-Type", "text/plain");
                    }
                    return Response::new("200", Some(headers), Some(contents));
                }
                return NotFoundHandler::handle(req);
            }
        }
    }
}

// api service handler
// TODO: 应该根据请求方法和路径再次分发到
// 不同的 cntroller, 实现类似 Rest 的 api 风格
// 比如 get /api/user -> UserApi::index
// 比如 post /api/user -> UserApi::cerate
// 这里就不实现了, 直接返回一个固定的 json 字符串
impl Handler for ApiHandler {
    fn handle(_req: &Request) -> Response {
        let mut headers: HashMap<&str, &str> = HashMap::new();
        headers.insert("Content-Type", "application/json");

        let json_str = String::from("{\"errno\":0,\"msg\":\"success\", \"data\": null}");

        Response::new("200", Some(headers), Some(json_str))
    }
}
