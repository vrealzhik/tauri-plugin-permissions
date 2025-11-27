use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResult {
  pub status: Option<bool>
}

    // @Command
    // fun requestAllPermissions(invoke: Invoke) {
    //     val permissions = mutableListOf<String>()

    //     permissions.add(Manifest.permission.ACCESS_FINE_LOCATION)

    //     if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
    //         permissions.add(Manifest.permission.BLUETOOTH_SCAN)
    //         permissions.add(Manifest.permission.BLUETOOTH_CONNECT)
    //     }

    //     if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
    //         permissions.add(Manifest.permission.POST_NOTIFICATIONS)
    //     }

    //     ActivityCompat.requestPermissions(
    //         activity,
    //         permissions.toTypedArray(),
    //         200
    //     )

    //     invoke.resolve(JSObject().apply { put("status", "requested") })
    // }