use std::net::ToSocketAddrs;
use surf::Client;
use tide::{
    http::{self, Url},
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
            .expect(&format!("could not parse {} as forwarding url", s)),
        )
    }
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Middleware<T> for ForwardMiddleware {
    async fn handle(&self, request: Request<T>, next: Next<'_, T>) -> Result {
        let http_request: &http::Request = request.as_ref();
        let mut http_request = http_request.clone();
        let response = next.run(request).await;
        if let Some(error) = response.error() {
            if error.status() == 404 {
                let url = http_request.url_mut();
                url.set_scheme(self.0.scheme())
                    .map_err(|_| http::format_err!("could not set scheme of url {}", url))?;
                url.set_host(self.0.host().map(|h| h.to_string()).as_deref())
                    .map_err(|_| http::format_err!("could not set host of url {}", url))?;
                url.set_port(self.0.port())
                    .map_err(|_| http::format_err!("could not set port of url {}", url))?;
                let http_response: http::Response = Client::new().send(http_request).await?.into();
                return Ok(http_response.into());
            }
        }

        Ok(response)
    }
}
