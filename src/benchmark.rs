use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use colored::Colorize;
use anyhow::Context;
use crate::{
    requester::{build_client, build_request},
    stats::{RequestStat, RequestStatSummary},
    CliArgs,
};

pub fn run_benchmark(
    rps: u64,
    duration: u64,
    args: CliArgs,
    urls: Vec<String>,
) -> anyhow::Result<RequestStatSummary> {
    let request_delta = Duration::from_secs_f64(1.0 / rps as f64);

    let mut threads = Vec::with_capacity(urls.len());
    let mut channels = Vec::with_capacity(urls.len());


    for url in urls {
        //FIXME: unwrap
        let client = build_client(&args).unwrap();
        let request =
            build_request(&url, &args, &client).context("Failed to build request for {url}")?;

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn::<_, anyhow::Result<()>>(move || {
            let start = Instant::now();

            let mut last_request_time = start;

            // for _ in 0..request_count as usize {
            while start.elapsed().as_secs() < duration {
                if last_request_time.elapsed() < request_delta {
                    thread::sleep(request_delta - last_request_time.elapsed());
                }

                let now = Instant::now();

                // Unwrap is ok, because body is not a stream
                // (probably)
                let res = request
                    .try_clone()
                    .context(format!("failed to build request for {url}"))?
                    .send()
                    .context(format!("failed to send request for {url}"))?;

                let duration = now.elapsed();

                let result = RequestStat::from(res).with_duration(duration);

                if !args.quiet {
                    println!("{result}");
                }

                tx.send(result).unwrap();

                last_request_time = Instant::now();
            }
            Ok(())
        });

        threads.push(handle);
        channels.push(rx);
    }

    threads
        .into_iter()
        .map(|x| x.join().unwrap_or(Err(anyhow::anyhow!("Thread panicked"))))
        .for_each(|result| match result {
            Ok(_) => {}
            Err(err) => {
                if args.verbose {
                    println!("{}", format!("\nError while processing target.\n\t{err}").red());
                }
            }
        });

    Ok(RequestStatSummary::from(channels))
}
