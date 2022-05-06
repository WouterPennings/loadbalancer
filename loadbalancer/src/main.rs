use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use std::convert::Infallible;

// HTTP Codes
static NOT_FOUND: &[u8] = b"404: Not Found";
static BAD_REQUEST: &[u8] = b"400: Bad Request";

// Index of the current server to proxy request to
static mut INDEX: i32 = 0;

unsafe fn get_ip() -> String {
    let ips = vec![
        String::from("http://localhost:3000"),
        String::from("http://localhost:3001"),
        String::from("http://localhost:3002"),
    ];
    INDEX += 1;
    if INDEX == ips.len() as i32 {
        INDEX = 0;
    }

    ips[INDEX as usize].clone()
}

async fn proxy_request(
    client: Client<HttpConnector>,
    uri: String,
) -> Result<Response<Body>, hyper::Error> {
    let res = client.get(uri.parse().unwrap()).await?;

    println!("LoadBalancer Response: {}", res.status());

    Ok(res)
}

async fn handle_request(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().clone();
    match path {
        "/" => Ok(Response::builder()
            .status(StatusCode::OK)
            .body("<h1>Welcome to the LoadBalancer prototype</h1>".into())
            .unwrap()),
        "/status" => Ok(Response::builder()
            .status(StatusCode::OK)
            .body("200: Webserver is OK".into())
            .unwrap()),
        _ => {
            // Actual loadbalancer request: "localhost:8000/lb/*"
            if path.to_string()[..3].to_string().as_str() == "/lb" {
                let path = req.uri().path().clone().to_string()[3..].to_string();
                let uri = unsafe { format!("{}{}", get_ip(), path) };
                println!("Current IP: {}", uri);
                let res = proxy_request(client, uri).await;
                if let Ok(e) = res {
                    // Successfully returning the response of the proxied server
                    return Ok(e);
                }
                // Returning "BAD REQUEST" as
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(BAD_REQUEST.into())
                    .unwrap())
            // unknown endpoint
            } else {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(NOT_FOUND.into())
                    .unwrap())
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Setting up the client to talk to the servers
    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    // Creating the service that handles the webserver requests
    let make_svc = make_service_fn(|_conn| {
        let client = client.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| handle_request(req, client.clone()))) }
    });

    // IP address from this webserver
    let addr = ([127, 0, 0, 1], 8000).into();

    // Initializing server
    let server = Server::bind(&addr)
        .http1_title_case_headers(true)
        .serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
