use crate::StatusUpdate;
use std::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn diskspace(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        let total = Command::new("sh")
            .arg("-c")
            .arg("df -B1 / | awk 'NR==2 {print $2}'")
            .output()
            .expect("Failed to get total space");
        let total_bytes = String::from_utf8(total.stdout).unwrap().trim().to_string();
        let total_f64: f64 = total_bytes.parse().unwrap();

        let avail = Command::new("sh")
            .arg("-c")
            .arg("df -B1 / | awk 'NR==2 {print $4}'")
            .output()
            .expect("Failed to get available space");
        let avail_bytes = String::from_utf8(avail.stdout).unwrap().trim().to_string();
        let avail_f64: f64 = avail_bytes.parse().unwrap();

        let avail_perc = (avail_f64 / total_f64) * 100.0;
        let used = total_f64 - avail_f64;
        let _used_perc = (used / total_f64) * 100.0;

        let _total_gb = total_f64 / 1_073_741_824.0;
        let _used_gb = used / 1_073_741_824.0;
        let avail_gb = avail_f64 / 1_073_741_824.0;

        let output = format!("['/': {:.1}GB ({:.0}%) Avail]", avail_gb, avail_perc);

        tx.send(StatusUpdate {
            module: "diskspace",
            text: format!("<span foreground=\"{}\">{}</span>", color, output),
        })
        .await
        .unwrap();

        sleep(Duration::from_secs(secs)).await;
    }
}
