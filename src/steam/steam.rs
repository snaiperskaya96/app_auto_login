use app_auto_login::{auto_login, winapi::SetFocus};
use clap::Parser;
use rustautogui::RustAutoGui;
use windows_registry::LOCAL_MACHINE;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    pass: String,
}

fn main() {
    auto_login::init();
    
    let cli = Cli::parse();

    auto_login::kill_all_processes_by_name("steam.exe");

    // nice.
    // (this is to avoid matching the steam logo from a closing window)
    std::thread::sleep(std::time::Duration::from_secs(2));

    log::info!("Looking for steam's installation path");

    const STEAM_PATH: &str = "SOFTWARE\\WOW6432Node\\Valve\\Steam";
    let steam_key = LOCAL_MACHINE.open(STEAM_PATH).expect("Could not retrieve Steam's registry key.");
    let install_path = steam_key.get_string("InstallPath").expect(format!("Could not retrieve InstallPath in {}", STEAM_PATH).as_str());
    log::info!("Found steam install path at {:?}", install_path);

    let steam_path = std::path::PathBuf::from(install_path).join("steam.exe");
    let mut cmd = std::process::Command::new(steam_path);
    let status = cmd.spawn().expect("msg");
    println!("{:?}", status);

    let r = auto_login::find_picture_in_process_window("steamwebhelper.exe", &std::env::current_exe().unwrap().parent().unwrap().join("steam_logo.png"), Some(std::time::Duration::from_secs(30)));
    let r = match r {
        Some(r) => r,
        None => {
            log::warn!("Could not find steam logo!");
            return;
        },
    };

    unsafe { SetFocus(r.window) };
    let rustautogui = RustAutoGui::new(false);
    rustautogui.keyboard_string(&cli.user);
    rustautogui.keyboard_command("tab");
    rustautogui.keyboard_string(&cli.pass);
    rustautogui.keyboard_command("enter");

}
