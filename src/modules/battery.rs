use crate::StatusUpdate;
use std::fs::File;
use std::io::{Seek, prelude::*};
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

pub async fn battery(
    tx: Sender<StatusUpdate>,
    battery_name: &str,
    secs: u64,
    mut color: &str,
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

    let mut capacity_contents = String::new();
    let mut status_contents = String::new();

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

        if status_colors && status_contents.trim() == "Discharging" {
            color = "red";
        } else if status_colors {
            color = "green";
        }

        tx.send(StatusUpdate {
            module: "battery".to_string(),
            text: format!("<span foreground=\"{}\">{}</span>", color, output),
        })
        .await
        .unwrap();

        capacity_contents.clear();
        status_contents.clear();

        sleep(Duration::from_secs(secs)).await;
    }
}
