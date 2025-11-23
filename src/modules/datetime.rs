use crate::StatusUpdate;
use chrono::Local;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn date(tx: Sender<StatusUpdate>, secs: u64) {
    loop {
        let output = Local::now().format("%A, %b %d");
        tx.send(StatusUpdate {
            module: "date".to_string(),
            text: output.to_string(),
        })
        .await
        .unwrap();
        sleep(Duration::from_secs(secs)).await;
    }
}

pub async fn time(tx: Sender<StatusUpdate>, secs: u64) {
    loop {
        let output = Local::now().format("%H:%M:%S");
        tx.send(StatusUpdate {
            module: "time".to_string(),
            text: output.to_string(),
        })
        .await
        .unwrap();
        sleep(Duration::from_secs(secs)).await;
    }
}
