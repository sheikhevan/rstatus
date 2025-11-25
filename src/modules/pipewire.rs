use crate::StatusUpdate;
use std::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn pipewire(tx: Sender<StatusUpdate>, secs: u64, color: &str, status_colors: bool) {
    let mut output = String::with_capacity(32);
    loop {
        let result = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
            .expect("Failed to launch wpctl");
        let result_str = String::from_utf8_lossy(&result.stdout);
        let parts: Vec<&str> = result_str.split_whitespace().collect();

        let volume = if parts.len() >= 2 {
            if let Ok(vol) = parts[1].parse::<f32>() {
                (vol * 100.0) as u32
            } else {
                0
            }
        } else {
            0
        };

        let is_muted = parts.len() >= 3 && parts[2].contains("MUTED");

        output.clear();
        output.push_str("[Vol: ");
        output.push_str(&volume.to_string());
        output.push('%');
        if is_muted {
            output.push_str(" [MUTED]");
        }
        output.push(']');

        let final_color = if status_colors && is_muted {
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
