use std::{collections::HashMap, fmt::Display, sync::mpsc::Receiver, time::Duration};

use reqwest::blocking::Response;

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
        let success = match value.error_for_status_ref() {
            Ok(_) => true,
            Err(_) => false,
        };

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
        write!(
            f,
            "{}\t{}\t{} bytes\t{:?}\t-> {}",
            self.proto, self.status, self.size, self.duration, self.url
        )
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
