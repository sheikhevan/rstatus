use crate::StatusUpdate;
use time::OffsetDateTime;
use time::format_description::FormatItem;
use time::macros::format_description;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

const DATE_FORMAT: &[FormatItem] =
    format_description!("[weekday repr:long], [month repr:short] [day padding:zero]");
const TIME_FORMAT: &[FormatItem] = format_description!("[hour]:[minute]:[second]");

pub async fn date(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        let now = match OffsetDateTime::now_local() {
            Ok(dt) => dt,
            Err(_) => OffsetDateTime::now_utc(),
        };

        let formatted = match now.format(&DATE_FORMAT) {
            Ok(f) => f,
            Err(_) => {
                format!("{}-{:02}-{:02}", now.year(), now.month() as u8, now.day())
            }
        };

        let output = format!("[{}]", formatted);
        tx.send(StatusUpdate {
            module: "date",
            text: format!("<span foreground=\"{}\">{}</span>", color, output),
        })
        .await
        .unwrap();
        sleep(Duration::from_secs(secs)).await;
    }
}

pub async fn time(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        let now = match OffsetDateTime::now_local() {
            Ok(dt) => dt,
            Err(_) => OffsetDateTime::now_utc(),
        };

        let formatted = match now.format(&TIME_FORMAT) {
            Ok(f) => f,
            Err(_) => {
                format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
            }
        };
        let output = format!("[{}]", formatted);
        tx.send(StatusUpdate {
            module: "time",
            text: format!("<span foreground=\"{}\">{}</span>", color, output),
        })
        .await
        .unwrap();
        sleep(Duration::from_secs(secs)).await;
    }
}
