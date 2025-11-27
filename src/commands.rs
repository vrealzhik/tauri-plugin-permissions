use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::PermissionsExt;
use crate::Result;

// #[command]
// pub(crate) async fn ping<R: Runtime>(
//     app: AppHandle<R>,
//     payload: PingRequest,
// ) -> Result<PingResponse> {
//     app.permissions().ping(payload)
// }

#[command]
pub(crate) async fn get_bluetooth_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionResult> {
    app.permissions().get_bluetooth_permission()
}

#[command]
pub(crate) async fn get_location_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionResult> {
    app.permissions().get_location_permission()
}

#[command]
pub(crate) async fn get_notification_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PermissionResult> {
    app.permissions().get_notification_permission()
}
