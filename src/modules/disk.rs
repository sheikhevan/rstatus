use crate::StatusUpdate;
use sysinfo::Disks;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn diskspace(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        let disks = Disks::new_with_refreshed_list();

        if let Some(disk) = disks.iter().find(|d| d.mount_point().to_str() == Some("/")) {
            let total = disk.total_space();
            let avail = disk.available_space();
            let avail_perc = (avail as f64 / total as f64) * 100.0;
            let used = total - avail;
            let _used_perc = (used as f64 / total as f64) * 100.0;

            let _total_gb = total as f64 / 1_073_741_824.0;
            let _used_gb = used as f64 / 1_073_741_824.0;
            let avail_gb = avail as f64 / 1_073_741_824.0;

            let output = format!("['/': {:.1}GB ({:.0}%) Avail]", avail_gb, avail_perc);

            tx.send(StatusUpdate {
                module: "diskspace".to_string(),
                text: format!("<span foreground=\"{}\">{}</span>", color, output),
            })
            .await
            .unwrap();
        }

        sleep(Duration::from_secs(secs)).await;
    }
}
