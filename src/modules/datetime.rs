use crate::StatusUpdate;
use chrono::Local;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn date(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        let output = format!("[{}]", Local::now().format("%A, %b %d"));
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
        let output = format!("[{}]", Local::now().format("%H:%M:%S"));
        tx.send(StatusUpdate {
            module: "time",
            text: format!("<span foreground=\"{}\">{}</span>", color, output),
        })
        .await
        .unwrap();
        sleep(Duration::from_secs(secs)).await;
    }
}
