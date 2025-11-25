use crate::StatusUpdate;
use nix::ifaddrs::getifaddrs;
use std::fs::File;
use std::io::{Seek, prelude::*};
use std::net::IpAddr;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};

fn escape_pango(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn get_network_ip(interface_name: &str) -> Option<String> {
    let ifaddrs = getifaddrs().ok()?;

    for ifaddr in ifaddrs {
        if ifaddr.interface_name == interface_name {
            if let Some(address) = ifaddr.address {
                if let Some(sockaddr) = address.as_sockaddr_in() {
                    let ip = IpAddr::V4(sockaddr.ip());
                    let ip_string = ip.to_string();

                    // This checks if the IP is a private address
                    if ip_string.starts_with("10.")
                        || ip_string.starts_with("192.168.")
                        || (ip_string.starts_with("172.") && {
                            let second_octet: u8 = ip_string.split('.').nth(1)?.parse().ok()?;
                            (16..=31).contains(&second_octet)
                        })
                    {
                        return Some(ip_string);
                    }
                }
            }
        }
    }

    None
}

fn get_wireless_ssid(interface_name: &str) -> Option<String> {
    use std::process::Command;

    let ssid = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "iw dev {} link | grep 'SSID:' | sed 's/.*SSID: //'",
            interface_name
        ))
        .output()
        .ok()?;

    let ssid_str = String::from_utf8(ssid.stdout).ok()?.trim().to_string();
    if ssid_str.is_empty() {
        None
    } else {
        Some(ssid_str)
    }
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
    let mut operstate_contents = String::with_capacity(4);

    loop {
        file_operstate.seek(std::io::SeekFrom::Start(0)).unwrap();

        file_operstate
            .read_to_string(&mut operstate_contents)
            .unwrap_or_else(|_| {
                panic!("Unable to read /sys/class/net/{}/operstate", interface_name)
            });

        let ip_str = get_network_ip(interface_name).unwrap_or_default();
        let ssid_str = get_wireless_ssid(interface_name).unwrap_or_default();

        let output = format!(
            "[{}: \"{}\" ({})]",
            escape_pango(interface_name),
            if ssid_str.is_empty() {
                "N/A".to_string()
            } else {
                escape_pango(&ssid_str)
            },
            escape_pango(&ip_str),
        );

        let final_color =
            if status_colors && (operstate_contents.trim() != "up" || ssid_str.is_empty()) {
                "red"
            } else if status_colors {
                "green"
            } else {
                color
            };

        tx.send(StatusUpdate {
            module: "network",
            text: format!("<span foreground=\"{}\">{}</span>", final_color, output),
        })
        .await
        .unwrap();

        operstate_contents.clear();

        sleep(Duration::from_secs(secs)).await;
    }
}
