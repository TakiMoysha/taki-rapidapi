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

    use super::*;

    #[tokio::test]
    async fn test_futures_pipeline() {
        let start = OffsetDateTime::now_utc();
        let pipeline = stream::repeat_with(OffsetDateTime::now_utc)
            .then(|_| fetch())
            .then(|_| transform());

        pipeline.take(4).count().await;

        let duration = OffsetDateTime::now_utc() - start;
        println!("duration: {}", duration.as_seconds_f32());
    }

    #[tokio::test]
    async fn test_futures_pipeline_with_cpus() {
        const NUM_CPUS: usize = 4;
        let start = OffsetDateTime::now_utc();
        // no longer await the futures
        let pipeline = stream::repeat_with(OffsetDateTime::now_utc)
            .map(|_| fetch())
            .buffered(NUM_CPUS)
            .map(|_| transform())
            .buffered(NUM_CPUS);

        println!(
            "pipeline started: {}",
            (OffsetDateTime::now_utc() - start).as_seconds_f32()
        );
        pipeline.take(4).count().await;

        println!(
            "\tfinal: {}",
            (OffsetDateTime::now_utc() - start).as_seconds_f32()
        );
    }
}
