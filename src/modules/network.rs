use crate::StatusUpdate;
use std::fs::File;
use std::io::{Seek, prelude::*};
use std::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

fn escape_pango(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub async fn network(
    tx: Sender<StatusUpdate>,
    interface_name: &str,
    secs: u64,
    color: &str,
    status_colors: bool,
) {
    let mut file_operstate = File::open(format!("/sys/class/net/{}/operstate", interface_name))
        .unwrap_or_else(|_| panic!("Unable to open /sys/class/net/{}/operstate", interface_name));
    let mut operstate_contents = String::new();

    loop {
        file_operstate.seek(std::io::SeekFrom::Start(0)).unwrap();

        file_operstate
            .read_to_string(&mut operstate_contents)
            .unwrap_or_else(|_| {
                panic!("Unable to read /sys/class/net/{}/operstate", interface_name)
            });

        let ip = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ip -4 -o addr show {} | awk '{{print $4}}' | cut -d/ -f1 | grep -E '^(10\\.|172\\.(1[6-9]|2[0-9]|3[01])\\.|192\\.168\\.)' | head -n1",
                interface_name
            ))
            .output()
            .expect("Failed to get IP");
        let ip_stdout = String::from_utf8(ip.stdout).unwrap().trim().to_string();

        let ssid = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "iw dev {} link | grep 'SSID:' | sed 's/.*SSID: //'",
                interface_name
            ))
            .output()
            .expect("Failed to get SSID");
        let ssid_stdout = String::from_utf8(ssid.stdout).unwrap().trim().to_string();

        let output = format!(
            "[{}: \"{}\" ({})]",
            escape_pango(interface_name),
            if ssid_stdout.is_empty() {
                "N/A".to_string()
            } else {
                escape_pango(&ssid_stdout)
            },
            escape_pango(&ip_stdout),
        );

        let final_color =
            if status_colors && (operstate_contents.trim() != "up" || ssid_stdout.is_empty()) {
                "red"
            } else if status_colors {
                "green"
            } else {
                color
            };

        tx.send(StatusUpdate {
            module: "network".to_string(),
            text: format!("<span foreground=\"{}\">{}</span>", final_color, output),
        })
        .await
        .unwrap();

        operstate_contents.clear();

        sleep(Duration::from_secs(secs)).await;
    }
}
