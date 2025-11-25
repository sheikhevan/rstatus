use crate::StatusUpdate;
use std::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn pipewire(tx: Sender<StatusUpdate>, secs: u64, color: &str, status_colors: bool) {
    loop {
        let volume = Command::new("sh")
            .arg("-c")
            .arg("wpctl get-volume @DEFAULT_AUDIO_SINK@ | awk '{print int($2 * 100)}'")
            .output()
            .expect("Failed to launch wpctl");
        let volume_stdout = String::from_utf8(volume.stdout).unwrap().trim().to_string();

        let muted = Command::new("sh")
            .arg("-c")
            .arg("wpctl get-volume @DEFAULT_AUDIO_SINK@ | awk '{print $3}'")
            .output()
            .expect("Failed to launch wpctl");
        let muted_stdout = String::from_utf8(muted.stdout).unwrap().trim().to_string();

        let output = format!(
            "[Vol: {}%{}]",
            volume_stdout,
            if muted_stdout.is_empty() {
                ""
            } else {
                " [MUTED]"
            }
        );

        let final_color = if status_colors && !muted_stdout.is_empty() {
            "red"
        } else {
            color
        };

        tx.send(StatusUpdate {
            module: "pipewire",
            text: format!("<span foreground=\"{}\">{}</span>", final_color, output),
        })
        .await
        .unwrap();

        sleep(Duration::from_secs(secs)).await;
    }
}
