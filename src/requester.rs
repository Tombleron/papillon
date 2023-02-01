use clap::ValueEnum;
use reqwest::{header, redirect::Policy, blocking::{RequestBuilder, Client}};
use std::{fs::read_to_string, net::ToSocketAddrs, time::Duration};

use crate::CliArgs;

#[derive(Debug, Clone, ValueEnum)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}

impl Method {
    fn to_request(&self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Trace => reqwest::Method::TRACE,
        }
    }
}

pub fn build_client(args: &CliArgs) -> color_eyre::Result<Client> {
    let client_builder = if args.no_http2 {
        Client::builder().http1_only()
    } else {
        Client::builder()
    };

    // TODO: enforce SSL
    // TODO: keepalive
    client_builder
        .gzip(args.compress)
        // .http2_keep_alive_interval(if args.keepalive {
            // FIXME: decide on interval value
        //     Some(Duration::from_secs(5))
        // } else {
        //     None
        // })
        .timeout(Duration::from_secs(args.timeout))
        .redirect(if args.follow_redirects {
            Policy::limited(10)
        } else {
            Policy::none()
        })
        .build()
        .map_err(From::from)
}

pub fn build_request(url: &str, args: &CliArgs, client: &Client) -> color_eyre::Result<RequestBuilder> {
    // let client_builder = reqwest::Client::builder();

    let request_url = if args.dns_prefetch {
        let mut res = url.to_socket_addrs()?;
        // TODO: unwrap
        res.next().unwrap().to_string()
    } else {
        url.to_owned()
    };

    // let client = client_builder.build()?;

    let mut request_builder = client.request(args.request_method.to_request(), request_url);

    // FIXME: this looks ugly
    request_builder = if let Some(path) = &args.body_file {
        let body = read_to_string(path)?;
        request_builder.body(body)
    } else if let Some(body) = &args.body {
        request_builder.body(body.clone())
    } else {
        request_builder
    };

    for (key, value) in &args.headers {
        request_builder = request_builder.header(key, value)
    }
    request_builder = request_builder.header("User-Agent", args.user_agent.clone());

    if let Some(cookies) = &args.cookies {
        request_builder = request_builder.header(header::COOKIE, cookies);
    }

    if let Some((username, password)) = &args.basic_auth {
        request_builder = request_builder.basic_auth(username, Some(password));
    }

    Ok(request_builder)
}
