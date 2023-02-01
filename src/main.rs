mod benchmark;
mod requester;
mod stats;

use std::error::Error;

use benchmark::run_benchmark;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "papillon")]
#[command(about="command line HTTP stress tester.", long_about=None)]
struct Cli {
    // Subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
pub struct CliArgs {
    // TODO: regex URLs
    // #[arg(
    //     short = 'r',
    //     help = "Interpret URLs as regular expressions.",
    //     default_value = "false"
    // )]
    // regex: bool,
    #[arg(
        long,
        help = "Prefetch IP from hostname before making request, eliminating DNS fetching from timing.",
        default_value = "false"
    )]
    dns_prefetch: bool,

    #[arg(
        short = 't',
        help = "Maximum seconds to wait for response",
        default_value = "10"
    )]
    timeout: u64,

    #[arg(
        short = 'X',
        help = "Request type. GET, HEAD, POST, PUT, etc.",
        value_enum,
        default_value = "GET",
        ignore_case = true
    )]
    request_method: requester::Method,

    #[arg(long, help = "String to use as request body e.g. POST body.")]
    body: Option<String>,

    #[arg(
        long,
        help = "Interpret Body as regular expressions.",
        default_value = "false"
    )]
    body_regex: bool,

    #[arg(long, help="Path to file to use as request body. Will overwrite --body if both are present.", value_hint = clap::ValueHint::DirPath)]
    body_file: Option<std::path::PathBuf>,

    #[arg(
        short = 'H',
        help = "Add arbitrary header line, eg. 'Accept-Encoding:gzip, Content-Type:application/json'",
        value_parser = parse_key_val::<String,String>,
        value_delimiter=','
    )]
    headers: Vec<(String, String)>,

    #[arg(long, help = "Add request cookies, eg. 'data=123; session=456'")]
    cookies: Option<String>,

    #[arg(
        short = 'A',
        help = "Add User-Agent header. Can also be done with the arbitrary header flag.",
        default_value = "papillon"
    )]
    user_agent: String,

    #[arg(long, help = "Add HTTP basic authentication, eg. 'user123:password456'.", value_parser = parse_key_val::<String,String>)]
    basic_auth: Option<(String, String)>,

    #[arg(
        short = 'C',
        help = "Add 'Accept-Encoding: gzip' header if Accept-Encoding is not already present.",
        default_value = "true"
    )]
    compress: bool,

    #[arg(short = 'k', default_value = "true", help = "Enable HTTP KeepAlive.")]
    keepalive: bool,

    #[arg(long, default_value = "true", help = "Follow HTTP redirects.")]
    follow_redirects: bool,

    #[arg(long, default_value = "false", help = "Disable HTTP2.")]
    no_http2: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Enfore SSL certificate correctness."
    )]
    enforce_ssl: bool,

    #[arg(long, value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as JSON")]
    output_json: Option<std::path::PathBuf>,

    #[arg(long, value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as CSV")]
    output_csv: Option<std::path::PathBuf>,

    #[arg(long, value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as XML")]
    output_xml: Option<std::path::PathBuf>,

    #[arg(
        short = 'q',
        default_value = "false",
        help = "Do not print while requests are running."
    )]
    quiet: bool,

    #[arg(
        short = 'v',
        default_value = "false",
        help = "Print extra troubleshooting info."
    )]
    verbose: bool,

    // TODO: default value
    #[arg(long, default_value = "1", help = "Number of CPUs to use.")]
    cpu: i32,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find(':')
        .ok_or_else(|| format!("invalid KEY:VALUE: no `:` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run benchmark tests", arg_required_else_help = true)]
    Benchmark {
        #[arg(long, help = "Requests per second to make.", default_value = "10")]
        rps: u64,

        // TODO: switch to Duration
        #[arg(
            help = "Number of seconds to send requests. Total benchmark test duration will be longer due to waiting for requests to finish.",
            default_value = "10",
            short = 'd'
        )]
        duration: u64,

        #[command(flatten)]
        args: CliArgs,

        // #[arg(help = "Benchmark targets", action=clap::ArgAction::Set, last=true)]
         #[arg(help = "Benchmark targets", last=true)]
        targets: Vec<String>,
    },

    #[command(arg_required_else_help = true)]
    Stress {
        #[command(flatten)]
        args: CliArgs,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Benchmark {
            rps,
            duration,
            args,
            targets,
        } => {
                let stats = run_benchmark(rps, duration, args, targets).unwrap();
                dbg!(stats);
        }
        Commands::Stress { args: _ } => todo!(),
    }
}
