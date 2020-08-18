use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

pub struct WebServer {
    pg_client: Option<postgres::Client>,
}

impl WebServer {
    pub fn new() -> Self {
        Self { pg_client: None }
    }

    pub fn db(mut self, pg_client: postgres::Client) -> Self {
        self.pg_client = Some(pg_client);
        self
    }

    pub async fn serve(&mut self, port: u16) {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let make_service = make_service_fn(|socket: &AddrStream| {
            println!("{}", socket.remote_addr());
            async move { Ok::<_, Infallible>(service_fn(handle)) }
        });

        let server = Server::bind(&addr).serve(make_service);

        println!("Listening on http://localhost:{}", port);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    //let rows = pg_client.query("SELECT * FROM list", &[]).unwrap();
    //rows.iter().for_each(|row| println!("{}", row.get::<_, &str>(1)));

    Ok(Response::new(Body::from("test")))
}
