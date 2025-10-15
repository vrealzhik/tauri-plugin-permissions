use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::{models::*, Result};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_permissions);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<Permissions<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.permissions", "PermissionsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_permissions)?;
    Ok(Permissions(handle))
}

/// Access to the permissions APIs.
pub struct Permissions<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Permissions<R> {
    pub async fn check_permissions(&self) -> Result<PermissionsResult> {
        let result: serde_json::Value = self.0.run_mobile_plugin("checkPermissions", ())?;

        Self::parse_permissions_result(&result)
    }

    pub async fn request_permissions(
        &self,
        request: PermissionRequest,
    ) -> Result<PermissionsResult> {
        let permissions_str: Vec<&str> = request.permissions.iter().map(|p| p.as_str()).collect();
        let request_data = serde_json::json!({ "permissions": permissions_str });

        let result: serde_json::Value =
            self.0.run_mobile_plugin("requestPermissions", request_data)?;

        Self::parse_permissions_result(&result)
    }

    pub async fn check_location_permission(&self) -> Result<CheckPermissionResponse> {
        let result: serde_json::Value = self.0.run_mobile_plugin("checkLocationPermission", ())?;

        Self::parse_check_permission_response(&result, "location")
    }

    pub async fn request_location_permission(&self) -> Result<RequestPermissionResponse> {
        let result: serde_json::Value = self.0.run_mobile_plugin("requestLocationPermission", ())?;

        Self::parse_request_permission_response(&result, "location")
    }

    pub async fn check_notifications_permission(&self) -> Result<CheckPermissionResponse> {
        let result: serde_json::Value =
            self.0.run_mobile_plugin("checkNotificationsPermission", ())?;

        Self::parse_check_permission_response(&result, "notifications")
    }

    pub async fn request_notifications_permission(&self) -> Result<RequestPermissionResponse> {
        let result: serde_json::Value = self
            .0
            .run_mobile_plugin("requestNotificationsPermission", ())?;

        Self::parse_request_permission_response(&result, "notifications")
    }

    pub async fn check_bluetooth_scan_permission(&self) -> Result<CheckPermissionResponse> {
        let result: serde_json::Value =
            self.0.run_mobile_plugin("checkBluetoothScanPermission", ())?;

        Self::parse_check_permission_response(&result, "bluetoothScan")
    }

    pub async fn request_bluetooth_scan_permission(&self) -> Result<RequestPermissionResponse> {
        let result: serde_json::Value = self
            .0
            .run_mobile_plugin("requestBluetoothScanPermission", ())?;

        Self::parse_request_permission_response(&result, "bluetoothScan")
    }

    pub async fn check_bluetooth_connect_permission(&self) -> Result<CheckPermissionResponse> {
        let result: serde_json::Value = self
            .0
            .run_mobile_plugin("checkBluetoothConnectPermission", ())?;

        Self::parse_check_permission_response(&result, "bluetoothConnect")
    }

    pub async fn request_bluetooth_connect_permission(&self) -> Result<RequestPermissionResponse> {
        let result: serde_json::Value = self
            .0
            .run_mobile_plugin("requestBluetoothConnectPermission", ())?;

        Self::parse_request_permission_response(&result, "bluetoothConnect")
    }

    // Вспомогательные методы для парсинга

    fn parse_permissions_result(result: &serde_json::Value) -> Result<PermissionsResult> {
        let mut permissions = Vec::new();
        let mut all_granted = true;

        if let Some(location_status) = result.get("location").and_then(|v| v.as_str()) {
            let (status, granted) = Self::parse_permission_status(location_status);
            if !granted {
                all_granted = false;
            }
            permissions.push(SinglePermissionResult {
                permission: PermissionType::Location,
                status,
                granted,
            });
        }

        if let Some(notifications_status) = result.get("notifications").and_then(|v| v.as_str()) {
            let (status, granted) = Self::parse_permission_status(notifications_status);
            if !granted {
                all_granted = false;
            }
            permissions.push(SinglePermissionResult {
                permission: PermissionType::Notifications,
                status,
                granted,
            });
        }

        if let Some(bluetooth_scan_status) = result.get("bluetoothScan").and_then(|v| v.as_str()) {
            let (status, granted) = Self::parse_permission_status(bluetooth_scan_status);
            if !granted {
                all_granted = false;
            }
            permissions.push(SinglePermissionResult {
                permission: PermissionType::BluetoothScan,
                status,
                granted,
            });
        }

        if let Some(bluetooth_connect_status) =
            result.get("bluetoothConnect").and_then(|v| v.as_str())
        {
            let (status, granted) = Self::parse_permission_status(bluetooth_connect_status);
            if !granted {
                all_granted = false;
            }
            permissions.push(SinglePermissionResult {
                permission: PermissionType::BluetoothConnect,
                status,
                granted,
            });
        }

        Ok(PermissionsResult {
            permissions,
            all_granted,
        })
    }

    fn parse_check_permission_response(
        result: &serde_json::Value,
        permission_key: &str,
    ) -> Result<CheckPermissionResponse> {
        let status_str = result
            .get(permission_key)
            .and_then(|v| v.as_str())
            .unwrap_or("notDetermined");

        let (status, granted) = Self::parse_permission_status(status_str);

        Ok(CheckPermissionResponse { status, granted })
    }

    fn parse_request_permission_response(
        result: &serde_json::Value,
        permission_key: &str,
    ) -> Result<RequestPermissionResponse> {
        let status = result
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let (permission_status, granted) = if status == "already_granted" {
            (PermissionStatus::Granted, true)
        } else {
            // Для запрошенных разрешений, статус будет приходить через события
            (PermissionStatus::NotDetermined, false)
        };

        Ok(RequestPermissionResponse {
            status: permission_status,
            granted,
            requested: status == "request_initiated",
        })
    }

    fn parse_permission_status(status_str: &str) -> (PermissionStatus, bool) {
        match status_str {
            "granted" => (PermissionStatus::Granted, true),
            "denied" => (PermissionStatus::Denied, false),
            "prompt" => (PermissionStatus::Prompt, false),
            "prompt-with-rationale" => (PermissionStatus::PromptWithRationale, false),
            _ => (PermissionStatus::NotDetermined, false),
        }
    }
}
