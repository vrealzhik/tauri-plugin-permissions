use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Permissions<R>> {
    Ok(Permissions(app.clone()))
}

/// Access to the permissions APIs.
pub struct Permissions<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Permissions<R> {
    pub fn get_bluetooth_permission(
        &self,
    ) -> crate::Result<PermissionResult> {
        Ok(PermissionResult {
            status: Some(String::default()),
        })
    }
    pub fn get_location_permission(
        &self,
    ) -> crate::Result<PermissionResult> {
        Ok(PermissionResult {
            status: Some(String::default()),
        })
    }
    pub fn get_notification_permission(
        &self,
    ) -> crate::Result<PermissionResult> {
        Ok(PermissionResult {
            status: Some(String::default()),
        })
    }
}
