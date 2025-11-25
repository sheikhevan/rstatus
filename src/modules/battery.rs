use crate::StatusUpdate;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

// INFO: This module takes two additional parameters: `battery_name` and `status_colors`.
// `battery_name` is simply the name of the battery as it is in `/sys/class/power_supply/` (for
// most people this will be `BAT0`). `status_colors` is a boolean that means if the battery is
// discharging the color will be red and if it is charging it will be green.

pub async fn battery(
    tx: Sender<StatusUpdate>,
    battery_name: &str,
    secs: u64,
    color: &str,
    status_colors: bool,
) {
    let file_capacity = File::open(format!("/sys/class/power_supply/{}/capacity", battery_name))
        .unwrap_or_else(|_| {
            panic!(
                "Unable to open /sys/class/power_supply/{}/capacity",
                battery_name
            )
        });
    let file_status = File::open(format!("/sys/class/power_supply/{}/status", battery_name))
        .unwrap_or_else(|_| {
            panic!(
                "Unable to open /sys/class/power_supply/{}/status",
                battery_name
            )
        });

    let mut reader_capacity = BufReader::new(file_capacity);
    let mut reader_status = BufReader::new(file_status);

    let mut capacity_line = String::with_capacity(8);
    let mut status_line = String::with_capacity(16);

    let mut output = String::with_capacity(32);

    loop {
        capacity_line.clear();
        status_line.clear();

        reader_capacity.seek(SeekFrom::Start(0)).unwrap();
        reader_status.seek(SeekFrom::Start(0)).unwrap();

        reader_capacity
            .read_line(&mut capacity_line)
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to read /sys/class/power_supply/{}/capacity",
                    battery_name
                )
            });
        reader_status
            .read_line(&mut status_line)
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to read /sys/class/power_supply/{}/status",
                    battery_name
                )
            });

        let capacity_trimmed = capacity_line.trim();
        let status_trimmed = status_line.trim();

        // We now reuse the output buffer
        output.clear();
        output.push('[');
        output.push_str(capacity_trimmed);
        output.push_str("% (");
        output.push_str(status_trimmed);
        output.push_str(")]");

        let final_color = if status_colors && status_trimmed == "Discharging" {
            "red"
        } else if status_colors {
            "green"
        } else {
            color
        };

        tx.send(StatusUpdate {
            module: "battery",
            text: format!("<span foreground=\"{}\">{}</span>", final_color, output),
        })
        .await
        .unwrap();

        sleep(Duration::from_secs(secs)).await;
    }
}
