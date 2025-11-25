use crate::StatusUpdate;
use std::fs::File;
use std::io::{Seek, prelude::*};
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
    let mut file_capacity =
        File::open(format!("/sys/class/power_supply/{}/capacity", battery_name)).unwrap_or_else(
            |_| {
                panic!(
                    "Unable to open /sys/class/power_supply/{}/capacity",
                    battery_name
                )
            },
        );
    let mut file_status = File::open(format!("/sys/class/power_supply/{}/status", battery_name))
        .unwrap_or_else(|_| {
            panic!(
                "Unable to open /sys/class/power_supply/{}/status",
                battery_name
            )
        });

    let mut capacity_contents = String::with_capacity(8);
    let mut status_contents = String::with_capacity(16);

    loop {
        file_capacity.seek(std::io::SeekFrom::Start(0)).unwrap();
        file_status.seek(std::io::SeekFrom::Start(0)).unwrap();
        file_capacity
            .read_to_string(&mut capacity_contents)
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to read /sys/class/power_supply/{}/capacity",
                    battery_name
                )
            });
        file_status
            .read_to_string(&mut status_contents)
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to read /sys/class/power_supply/{}/status",
                    battery_name
                )
            });
        let output = format!(
            "[{}% ({})]",
            capacity_contents.trim(),
            status_contents.trim()
        );

        let final_color = if status_colors && status_contents.trim() == "Discharging" {
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

        capacity_contents.clear();
        status_contents.clear();

        sleep(Duration::from_secs(secs)).await;
    }
}
