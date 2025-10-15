use tauri::{command, AppHandle, Runtime};

use crate::{PermissionsExt, Result};
use crate::models::*;

#[command]
pub async fn check_permissions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionsResult> {
    app.permissions().check_permissions().await
}

#[command]
pub async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    request: PermissionRequest,
) -> Result<PermissionsResult> {
    app.permissions().request_permissions(request).await
}

#[command]
pub async fn check_location_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CheckPermissionResponse> {
    app.permissions().check_location_permission().await
}

#[command]
pub async fn request_location_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<RequestPermissionResponse> {
    app.permissions().request_location_permission().await
}

#[command]
pub async fn check_notifications_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CheckPermissionResponse> {
    app.permissions().check_notifications_permission().await
}

#[command]
pub async fn request_notifications_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<RequestPermissionResponse> {
    app.permissions().request_notifications_permission().await
}

#[command]
pub async fn check_bluetooth_scan_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CheckPermissionResponse> {
    app.permissions().check_bluetooth_scan_permission().await
}

#[command]
pub async fn request_bluetooth_scan_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<RequestPermissionResponse> {
    app.permissions().request_bluetooth_scan_permission().await
}

#[command]
pub async fn check_bluetooth_connect_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<CheckPermissionResponse> {
    app.permissions().check_bluetooth_connect_permission().await
}

#[command]
pub async fn request_bluetooth_connect_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<RequestPermissionResponse> {
    app.permissions().request_bluetooth_connect_permission().await
}