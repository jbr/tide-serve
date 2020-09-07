use http_client::{h1::H1Client, HttpClient};
use std::net::ToSocketAddrs;

use tide::{
    http::{self, proxies::Forwarded, Url},
    Middleware, Next, Request, Result,
};

pub struct ForwardMiddleware(Url);

impl ForwardMiddleware {
    pub fn new(s: &str) -> Self {
        Self(
            Url::parse(
                match s.to_socket_addrs() {
                    Ok(_) => format!("http://{}", s),
                    Err(_) => String::from(s),
                }
                .as_ref(),
            )
            .unwrap_or_else(|_| panic!("could not parse {} as forwarding url", s)),
        )
    }
}

fn forwarded(http_request: &http::Request) -> Forwarded<'static> {
    let mut forwarded = Forwarded::from_headers(http_request.as_ref())
        .ok()
        .flatten()
        .map(|f| f.into_owned())
        .unwrap_or_else(Forwarded::new);
    forwarded.add_for(http_request.peer_addr().unwrap_or("unknown"));
    forwarded.set_by("_tide_serve");
    forwarded.set_proto(http_request.url().scheme());
    forwarded.into_owned()
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Middleware<T> for ForwardMiddleware {
    async fn handle(&self, mut request: Request<T>, next: Next<'_, T>) -> Result {
        let body = request.take_body();
        let http_request: &http::Request = request.as_ref();
        let mut http_request = http_request.clone();
        http_request.set_body(body);
        let response = next.run(request).await;
        if response.status().is_client_error() {
            let forwarded = forwarded(&http_request);
            forwarded.apply(&mut http_request);

            let url = http_request.url_mut();
            url.set_scheme(self.0.scheme())
                .map_err(|_| http::format_err!("could not set scheme of url {}", url))?;
            url.set_host(self.0.host().map(|h| h.to_string()).as_deref())
                .map_err(|_| http::format_err!("could not set host of url {}", url))?;
            url.set_port(self.0.port())
                .map_err(|_| http::format_err!("could not set port of url {}", url))?;

            return Ok(H1Client::new().send(http_request).await?.into());
        }

        Ok(response)
    }
}
