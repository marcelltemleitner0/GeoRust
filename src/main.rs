use idevice::{IdeviceService, lockdown::LockdownClient};
mod common;

#[tokio::main]
async fn main() {
    print_banner();

    let udid = None;
    let host = None;
    let pairing_file = None;

    println!("🔍 Initializing device connection...");

    let provider = match common::get_provider(udid, host, pairing_file, "GeoRust").await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to get device provider:\n{e}");
            return;
        }
    };

    println!("📡 Connecting to lockdown service...");

    let mut lockdown_client = match LockdownClient::connect(&*provider).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Unable to connect to lockdown:\n{e:?}");
            return;
        }
    };

    match lockdown_client.get_value("ProductType").await {
        Ok(product_type) => {
            if let Some(v) = product_type.as_string() {
                println!("iOS Version: {}", v);
            }
        }
        Err(e) => eprintln!("❌ Failed to get ProductVersion:\n{e:?}"),
    };
}
fn print_banner() {
    let orange = "\x1b[38;5;208m";
    let reset = "\x1b[0m";
    println!(
        r#"{orange}


 ________   _______    ________   ________   ___  ___   ________   _________
|\   ____\ |\  ___ \  |\   __  \ |\   __  \ |\  \|\  \ |\   ____\ |\___   ___\
\ \  \___| \ \   __/| \ \  \|\  \\ \  \|\  \\ \  \\\  \\ \  \___|_\|___ \  \_|
 \ \  \  ___\ \  \_|/__\ \  \\\  \\ \   _  _\\ \  \\\  \\ \_____  \    \ \  \
  \ \  \|\  \\ \  \_|\ \\ \  \\\  \\ \  \\  \|\ \  \\\  \\|____|\  \    \ \  \
   \ \_______\\ \_______\\ \_______\\ \__\\ _\ \ \_______\ ____\_\  \    \ \__\
    \|_______| \|_______| \|_______| \|__|\|__| \|_______||\_________\    \|__|
                                                          \|_________|


                🔧 iOS CLI Toolkit
        {reset}"#
    );
}
