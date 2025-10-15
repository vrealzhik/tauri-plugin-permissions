const COMMANDS: &[&str] = &[
    "checkPermissions",
    "requestPermissions",
    "checkLocationPermission",
    "requestLocationPermission",
    "checkNotificationsPermission",
    "requestNotificationsPermission",
    "checkBluetoothScanPermission",
    "requestBluetoothScanPermission",
    "checkBluetoothConnectPermission",
    "requestBluetoothConnectPermission",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
