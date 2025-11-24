use std::io::Write;
use tokio::sync::mpsc;

mod modules;

pub struct StatusUpdate {
    pub module: String,
    pub text: String,
}

// INFO: To see available modules go to `src/modules`. To add or remove modules from your bar,
// simply add or remove their `tokio::spawn()` line. All functions require at least three
// parameters, tx (required; use `tx.clone()`), secs (the amount of time in between module
// reloads), and color (pango markup colors; if you just want the default color, use `"white"`).

// To see the specific parameters required for a module, go to it's file in `src/modules/`.

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    let mut status: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    tokio::spawn(crate::modules::network::network(
        tx.clone(),
        "wlp1s0f0",
        1,
        "white",
        true,
    ));
    tokio::spawn(crate::modules::pipewire::pipewire(
        tx.clone(),
        1,
        "white",
        true,
    ));
    tokio::spawn(crate::modules::disk::diskspace(tx.clone(), 1, "white"));
    tokio::spawn(crate::modules::battery::battery(
        tx.clone(),
        "macsmc-battery",
        1,
        "white", // NOTE: This is irrelevant since status_colors is set to true
        true,
    ));
    tokio::spawn(crate::modules::datetime::date(tx.clone(), 1, "white"));
    tokio::spawn(crate::modules::datetime::time(tx.clone(), 1, "white"));

    while let Some(update) = rx.recv().await {
        status.insert(update.module.to_string(), update.text.to_string());

        let mut actual_outputs: Vec<String> = Vec::new();

        if let Some(network) = status.get("network") {
            actual_outputs.push(network.to_string());
        }
        if let Some(pipewire) = status.get("pipewire") {
            actual_outputs.push(pipewire.to_string());
        }
        if let Some(disk) = status.get("diskspace") {
            actual_outputs.push(disk.to_string());
        }
        if let Some(battery) = status.get("battery") {
            actual_outputs.push(battery.to_string());
        }
        if let Some(date) = status.get("date") {
            actual_outputs.push(date.to_string());
        }
        if let Some(time) = status.get("time") {
            actual_outputs.push(time.to_string());
        }

        println!("{}", actual_outputs.join(" "));

        std::io::stdout().flush().unwrap();
    }
}
