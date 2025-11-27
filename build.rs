const COMMANDS: &[&str] = &[
    "get_notification_permission",
    "get_bluetooth_permission",
    "get_location_permission",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
