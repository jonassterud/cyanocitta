#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
pub async fn main() {
    cyanocitta::AppBuilder::new().run().await;
}
