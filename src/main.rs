use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "papillon")]
#[command(about="command line HTTP stress tester.", long_about=None)]
struct Cli {
    #[command(flatten)]
    args: CliArgs,

    // Subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct CliArgs {
    #[arg(
        short = 'r',
        help = "Interpret URLs as regular expressions.",
        default_value = "false"
    )]
    regex: bool,

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
    timeout: i32,

    #[arg(
        short = 'X',
        help = "Request type. GET, HEAD, POST, PUT, etc.",
        default_value = "GET"
    )]
    request_method: String,

    #[arg(help = "String to use as request body e.g. POST body.")]
    body: Option<String>,

    #[arg(
        long,
        help = "Interpret Body as regular expressions.",
        default_value = "false"
    )]
    body_regex: bool,

    #[arg(help="Path to file to use as request body. Will overwrite --body if both are present.", value_hint = clap::ValueHint::DirPath)]
    body_file: Option<std::path::PathBuf>,

    #[arg(
        short = 'H',
        help = "Add arbitrary header line, eg. 'Accept-Encoding:gzip, Content-Type:application/json'"
    )]
    headers: Option<String>,

    #[arg(help = "Add request cookies, eg. 'data=123; session=456'")]
    cookies: Option<String>,

    #[arg(
        short = 'A',
        help = "Add User-Agent header. Can also be done with the arbitrary header flag.",
        default_value = "pillon"
    )]
    user_agent: String,

    #[arg(help = "Add HTTP basic authentication, eg. 'user123:password456'.")]
    basic_auth: Option<String>,

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

    #[arg(value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as JSON")]
    output_json: Option<std::path::PathBuf>,

    #[arg(value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as CSV")]
    output_csv: Option<std::path::PathBuf>,

    #[arg(value_hint = clap::ValueHint::DirPath, help="Path to file to write full data as XML")]
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
    #[arg(default_value = "1", help = "Number of CPUs to use.")]
    cpu: i32,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run benchmark tests")]
    Benchmark {
        #[arg(long, help = "Requests per second to make.", default_value = "10")]
        rps: i32,

        // TODO: swithc to Duration
        #[arg(
            help = "Number of seconds to send requests. Total benchmark test duration will be longer due to waiting for requests to finish.",
            default_value = "10",
            short = 'd'
        )]
        duration: i32,

        #[arg(help = "Benchmark targets")]
        targets: Vec<String>,
    },

    #[command(arg_required_else_help = true)]
    Stress,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Benchmark {
            rps: _,
            duration: _,
            targets: _,
        } => todo!(),
        Commands::Stress => todo!(),
    }
}
