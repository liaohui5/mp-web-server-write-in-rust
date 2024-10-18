use mp_web_server::server::Server;

fn main() {
    let addr = "127.0.0.1:3000";
    let server = Server::new(addr);
    server.run();
}
