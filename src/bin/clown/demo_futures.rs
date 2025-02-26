use std::time::Duration;
use time::OffsetDateTime;

use tokio::time::sleep;

// keeping the I/O busy
pub async fn fetch() {
    sleep(Duration::from_secs(1)).await;
}

// keeping the CPU busy
pub async fn transform() {
    let end = OffsetDateTime::now_utc() + Duration::from_secs(1);
    while OffsetDateTime::now_utc() <= end {}
}

#[cfg(test)]
mod futures_tests {
    use futures::stream;
    use futures::stream::StreamExt;
    use throbber::Throbber;

    use super::*;

    #[tokio::test]
    async fn should_run_concurrent() {
        let mut throbber = Throbber::new().message("processing ...".to_string());
        let start = OffsetDateTime::now_utc();
        let pipeline = stream::repeat_with(OffsetDateTime::now_utc)
            .then(|_| fetch())
            .then(|_| transform());

        throbber.start();
        pipeline.take(4).count().await;

        let duration = OffsetDateTime::now_utc() - start;
        throbber.success(format!("duration: {}\n", duration.as_seconds_f32()));
    }

    #[tokio::test]
    async fn should_run_parallel() {
        let mut throbber = Throbber::new().message("processing ...".to_string());
        const NUM_CPUS: usize = 4;
        let start = OffsetDateTime::now_utc();
        // no longer await the futures
        let pipeline = stream::repeat_with(OffsetDateTime::now_utc)
            .map(|_| fetch())
            .buffered(NUM_CPUS)
            .map(|_| transform())
            .buffered(NUM_CPUS);

        throbber.start();
        pipeline.take(NUM_CPUS).count().await;

        let duration = OffsetDateTime::now_utc() - start;
        throbber.success(format!("duration: {}\n", duration.as_seconds_f32()));
    }
}
