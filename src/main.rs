use tokio::runtime;

mod config;
mod db;
mod server;

fn main() {
    let config = config::load_file("./todo.conf");
    let constr = config.get_must("postgres.constr");

    let pg_client = match db::connect(&constr) {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let mut web_server = server::WebServer::new().db(pg_client);

    let mut rt = runtime::Builder::new()
        .basic_scheduler()
        .enable_io()
        .build()
        .unwrap();

    let port = config.get_i32_or("server.port", 8000) as u16;

    rt.block_on(web_server.serve(port));
}
