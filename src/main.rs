use std::io::Write;
use tokio::sync::mpsc;

mod modules;

pub struct StatusUpdate {
    pub module: String,
    pub text: String,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    let mut status: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    tokio::spawn(crate::modules::datetime::date(tx.clone(), 1));
    tokio::spawn(crate::modules::datetime::time(tx.clone(), 1));

    while let Some(update) = rx.recv().await {
        status.insert(update.module.to_string(), update.text.to_string());

        let mut actual_outputs: Vec<String> = Vec::new();

        if let Some(date) = status.get("date") {
            actual_outputs.push(format!("[{}]", date));
        }
        if let Some(time) = status.get("time") {
            actual_outputs.push(format!("[{}]", time));
        }

        println!("{}", actual_outputs.join(" "));

        std::io::stdout().flush().unwrap();
    }
}
