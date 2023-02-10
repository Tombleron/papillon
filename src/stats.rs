use colored::Colorize;
use reqwest::blocking::Response;
use std::{collections::HashMap, fmt::Display, sync::mpsc::Receiver, time::Duration};

#[derive(Default, Debug)]
pub struct RequestStat {
    pub proto: String,
    pub url: String,
    // method: String,
    pub duration: Duration,
    pub status: u16,
    pub success: bool,
    pub size: u64,
}

impl RequestStat {
    pub fn with_duration(self, duration: Duration) -> Self {
        Self { duration, ..self }
    }
}

impl From<Response> for RequestStat {
    fn from(value: Response) -> Self {
        let success = value.error_for_status_ref().is_ok();

        Self {
            proto: format!("{:?}", value.version()),
            url: value.url().to_string(),
            status: value.status().as_u16(),
            success,
            size: value.content_length().unwrap_or(0),
            ..Default::default()
        }
    }
}

impl Display for RequestStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "{}\t{}\t{} bytes\t{:?}\t-> {}",
            self.proto, self.status, self.size, self.duration, self.url
        );

        match self.success {
            true => write!(f, "{}", string.green()),
            false => write!(f, "{}", string.red()),
        }
    }
}

#[derive(Debug, Default)]
pub struct RequestStatSummary {
    avg_rps: f64,

    avg_duration: Duration,
    min_duration: Duration,
    max_duration: Duration,

    statuse_codes: HashMap<u16, u32>,

    avg_query_size: u64,
    max_query_size: u64,
    min_query_size: u64,
    total_query_size: u64,
}

impl From<Vec<Receiver<RequestStat>>> for RequestStatSummary {
    fn from(channels: Vec<Receiver<RequestStat>>) -> Self {
        let mut summary = Self {
            min_duration: Duration::MAX,
            ..Default::default()
        };

        let mut total_duration = Duration::default();
        let mut total_query_size = 0;
        let mut request_count = 0;

        for rx in channels {
            for message in rx {
                total_duration += message.duration;

                summary.min_duration = message.duration.min(summary.min_duration);
                summary.max_duration = message.duration.max(summary.max_duration);

                summary
                    .statuse_codes
                    .entry(message.status)
                    .and_modify(|x| {
                        *x += 1;
                    })
                    .or_insert(1);

                total_query_size += message.size;

                summary.min_query_size = message.size.min(summary.min_query_size);
                summary.max_query_size = message.size.max(summary.max_query_size);

                request_count += 1;
            }
        }

        summary.avg_duration = total_duration / request_count;
        summary.avg_query_size = total_query_size / request_count as u64;
        summary.total_query_size = total_query_size;
        summary.avg_rps = 1.0 / summary.avg_duration.as_secs_f64();

        summary
    }
}

impl Display for RequestStatSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Timing\n\
            Mean query speed:\t{:?}\n\
            Fastest query speed:\t{:?}\n\
            Slowest query speed:\t{:?}\n\
            Mean RPS:\t{}\n\n\
            Data transfered\n\
            Mean query:\t{}\n\
            Largest query:\t{}\n\
            Smallest query:\t{}\n\
            Total Query:\t{}\n\n\
            Codes:\n",
            self.avg_duration,
            self.min_duration,
            self.max_duration,
            self.avg_rps,
            self.avg_query_size,
            self.min_query_size,
            self.max_query_size,
            self.total_query_size,
        )?;
        let mut status_vec: Vec<(u16, u32)> = self.statuse_codes.clone().into_iter().collect();
        status_vec.sort_by(|a, b| a.0.cmp(&b.0));
        for (k, v) in status_vec {
            writeln!(f, "{}: {}", k, v)?;
        }

        Ok(())
    }
}
