use crate::StatusUpdate;
use nix::sys::statvfs::statvfs;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn diskspace(tx: Sender<StatusUpdate>, secs: u64, color: &str) {
    loop {
        match statvfs("/") {
            Ok(stats) => {
                let block_size = stats.block_size() as f64;
                let total_blocks = stats.blocks() as f64;
                let avail_blocks = stats.blocks_available() as f64;

                let total_bytes = total_blocks * block_size;
                let avail_bytes = avail_blocks * block_size;
                let _used_bytes = total_bytes - avail_bytes;

                let avail_gb = avail_bytes / 1_073_741_824.0;
                let avail_perc = (avail_bytes / total_bytes) * 100.0;

                let output = format!("['/': {:.1}GB ({:.0}%) Avail]", avail_gb, avail_perc);

                tx.send(StatusUpdate {
                    module: "diskspace",
                    text: format!("<span foreground=\"{}\">{}</span>", color, output),
                })
                .await
                .unwrap();
            }
            Err(e) => {
                eprintln!("Failed to get disk stats: {}", e);
                tx.send(StatusUpdate {
                    module: "diskspace",
                    text: format!("<span foreground=\"{}\">[Disk: Error]</span>", color),
                })
                .await
                .unwrap();
            }
        }

        sleep(Duration::from_secs(secs)).await;
    }
}
