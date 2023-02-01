use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

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
) -> color_eyre::Result<RequestStatSummary> {
    let request_delta = Duration::from_secs_f64(1.0 / rps as f64);
    // let request_count = duration * rps;

    let mut threads = Vec::with_capacity(urls.len());
    let mut channels = Vec::with_capacity(urls.len());

    // let benchmark_start = Instant::now();

    for url in urls {
        //FIXME: unwrap
        let client = build_client(&args).unwrap();
        let request = build_request(&url, &args, &client).unwrap();

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
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
                    .unwrap()
                    .send()
                    // TODO: This unwrap is not ok
                    .unwrap();

                let duration = now.elapsed();

                let result = RequestStat::from(res).with_duration(duration);

                if !args.quiet {
                    println!("{result}");
                }

                tx.send(result).unwrap();

                last_request_time = Instant::now();
            }
        });

        threads.push(handle);
        channels.push(rx);
    }

    // let total_duration = benchmark_start.elapsed();

    threads.into_iter().map(|x| x.join()).for_each(drop);

    Ok(RequestStatSummary::from(channels))
}
