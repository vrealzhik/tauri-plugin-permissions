use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{models::*, Result};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Permissions<R>> {
    Ok(Permissions(app.clone()))
}

/// Access to the permissions APIs.
pub struct Permissions<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Permissions<R> {
    pub async fn check_permissions(&self) -> Result<PermissionsResult> {
        // На десктопе все разрешения считаются предоставленными
        Ok(PermissionsResult {
            permissions: vec![
                SinglePermissionResult {
                    permission: PermissionType::Location,
                    status: PermissionStatus::Granted,
                    granted: true,
                },
                SinglePermissionResult {
                    permission: PermissionType::Notifications,
                    status: PermissionStatus::Granted,
                    granted: true,
                },
                SinglePermissionResult {
                    permission: PermissionType::BluetoothScan,
                    status: PermissionStatus::Granted,
                    granted: true,
                },
                SinglePermissionResult {
                    permission: PermissionType::BluetoothConnect,
                    status: PermissionStatus::Granted,
                    granted: true,
                },
            ],
            all_granted: true,
        })
    }

    pub async fn request_permissions(&self, _request: PermissionRequest) -> Result<PermissionsResult> {
        // На десктопе просто возвращаем granted статус
        self.check_permissions().await
    }

    pub async fn check_location_permission(&self) -> Result<CheckPermissionResponse> {
        Ok(CheckPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
        })
    }

    pub async fn request_location_permission(&self) -> Result<RequestPermissionResponse> {
        Ok(RequestPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
            requested: false, // Не запрашивали, т.к. уже есть
        })
    }

    pub async fn check_notifications_permission(&self) -> Result<CheckPermissionResponse> {
        Ok(CheckPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
        })
    }

    pub async fn request_notifications_permission(&self) -> Result<RequestPermissionResponse> {
        Ok(RequestPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
            requested: false,
        })
    }

    pub async fn check_bluetooth_scan_permission(&self) -> Result<CheckPermissionResponse> {
        Ok(CheckPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
        })
    }

    pub async fn request_bluetooth_scan_permission(&self) -> Result<RequestPermissionResponse> {
        Ok(RequestPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
            requested: false,
        })
    }

    pub async fn check_bluetooth_connect_permission(&self) -> Result<CheckPermissionResponse> {
        Ok(CheckPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
        })
    }

    pub async fn request_bluetooth_connect_permission(&self) -> Result<RequestPermissionResponse> {
        Ok(RequestPermissionResponse {
            status: PermissionStatus::Granted,
            granted: true,
            requested: false,
        })
    }
}