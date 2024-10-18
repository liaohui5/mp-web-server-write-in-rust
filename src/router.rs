use crate::handler::{ApiHandler, Handler, NotFoundHandler, StaticFileHandler};
use crate::{request, response::Response};
use std::io::{Read, Write};

pub struct Router;

impl Router {
    #[allow(unused)]
    pub fn route<T: Read + Write>(req: request::Request, stream: &mut T) {
        // 此处只处理 GET 请求凡事
        if let request::Method::Get = req.method {
            // 结构方式获取请求路径
            let request::Resource::Path(s) = &req.resource;
            let route: Vec<&str> = s.split("/").collect();

            // 根据 handler 获取响应
            let response: Response = if route[1] == "api" {
                ApiHandler::handle(&req)
            } else {
                StaticFileHandler::handle(&req)
            };

            // 发送响应
            response.send(stream).unwrap();
            return;
        }

        // 如果不是 GET 请求就直接使用 NotFound
        NotFoundHandler::handle(&req).send(stream).unwrap();
    }
}
