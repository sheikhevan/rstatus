use std::io::{self, Write};
use tokio::sync::mpsc;

mod modules;

pub struct StatusUpdate {
    pub module: &'static str,
    pub text: String,
}

const ESTIMATED_MODULE_OUTPUT_SIZE: usize = 50;
const NUM_MODULES: usize = 6; // NOTE: Change this as you add more modules!

// INFO: To see available modules go to `src/modules`. To add or remove modules from your bar,
// simply add or remove their `tokio::spawn()` line. All functions require at least three
// parameters, tx (required; use `tx.clone()`), secs (the amount of time in between module
// reloads), and color (pango markup colors; if you just want the default color, use `"white"`).

// To see the specific parameters required for a module, go to it's file in `src/modules/`.

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8); // NOTE: Change this as you add more modules!

    let mut status: [Option<String>; NUM_MODULES] = Default::default();

    let mut output = String::with_capacity(NUM_MODULES * ESTIMATED_MODULE_OUTPUT_SIZE);

    tokio::spawn(crate::modules::network::network(
        tx.clone(),
        "wlan0",
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
    tokio::spawn(crate::modules::disk::diskspace(tx.clone(), 5, "white"));
    tokio::spawn(crate::modules::battery::battery(
        tx.clone(),
        "macsmc-battery",
        2,
        "white",
        true,
    ));
    tokio::spawn(crate::modules::datetime::date(tx.clone(), 1, "white"));
    tokio::spawn(crate::modules::datetime::time(tx.clone(), 1, "white"));

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(update) = rx.recv().await {
        // Map the module name to it's index
        let idx = match update.module {
            "network" => 0,
            "pipewire" => 1,
            "diskspace" => 2,
            "battery" => 3,
            "date" => 4,
            "time" => 5,
            _ => continue,
        };

        status[idx] = Some(update.text);

        // INFO: I did this for better mem usage; I clear and reuse the buffer instead of
        // allocating new strings like in the original code
        output.clear();

        let mut first = true;
        for item in &status {
            if let Some(text) = item {
                if !first {
                    output.push(' ');
                }
                output.push_str(text);
                first = false;
            }
        }

        let _ = writeln!(handle, "{}", output);
        let _ = handle.flush();
    }
}
