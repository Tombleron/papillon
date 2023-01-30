use crate::target::Target;
use clap::ValueEnum;
use reqwest::{header, redirect::Policy, Client, Request, Response};
use std::{fs::read_to_string, net::ToSocketAddrs, time::Duration};

#[derive(Debug, Clone, ValueEnum)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        match self {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
            Method::HEAD => reqwest::Method::HEAD,
            Method::OPTIONS => reqwest::Method::OPTIONS,
            Method::CONNECT => reqwest::Method::CONNECT,
            Method::PATCH => reqwest::Method::PATCH,
            Method::TRACE => reqwest::Method::TRACE,
        }
    }
}

fn build_client(target: &Target) -> color_eyre::Result<Client> {
    let client_builder = if target.args.no_http2 {
        reqwest::Client::builder().http1_only()
    } else {
        reqwest::Client::builder()
    };

    // TODO: enforce SSL

    client_builder
        .gzip(target.args.compress)
        .http2_keep_alive_interval(if target.args.keepalive {
            // FIXME: decide on interval value
            Some(Duration::from_secs(5))
        } else {
            None
        })
        .timeout(Duration::from_secs(target.args.timeout))
        .redirect(if target.args.follow_redirects {
            Policy::limited(10)
        } else {
            Policy::none()
        })
        .build()
        .map_err(From::from)
}

fn build_request(mut target: Target, client: Client) -> color_eyre::Result<Request> {
    // let client_builder = reqwest::Client::builder();

    if target.args.dns_prefetch {
        let mut res = target.url.to_socket_addrs()?;
        // TODO: unwrap
        target.url = res.next().unwrap().to_string();
    }

    // let client = client_builder.build()?;

    let mut request_builder = client.request(target.args.request_method.into(), target.url);

    // FIXME: this looks ugly
    request_builder = if let Some(path) = target.args.body_file {
        let body = read_to_string(path)?;
        request_builder.body(body)
    } else if let Some(body) = target.args.body {
        request_builder.body(body)
    } else {
        request_builder
    };

    for (key, value) in target.args.headers {
        request_builder = request_builder.header(key, value)
    }
    request_builder = request_builder.header("User-Agent", target.args.user_agent);

    if let Some(cookies) = target.args.cookies {
        request_builder = request_builder.header(header::COOKIE, cookies);
    }

    if let Some((username, password)) = target.args.basic_auth {
        request_builder = request_builder.basic_auth(username, Some(password));
    }

    Ok(request_builder.build()?)
}

// fn request_worker(client: Client) -> Response {

// }
