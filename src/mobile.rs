use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.permissions";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_permissions);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Permissions<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "PermissionsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_permissions)?;
    Ok(Permissions(handle))
}

/// Access to the permissions APIs.
pub struct Permissions<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Permissions<R> {
    pub fn get_bluetooth_permission(&self) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestBluetoothPermissions", ())
            .map_err(Into::into)
    }

    pub fn get_location_permission(&self) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestLocationPermission", ())
            .map_err(Into::into)
    }

    pub fn get_notification_permission(&self) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestNotificationPermission", ())
            .map_err(Into::into)
    }
    // pub fn get_all_permissions(&self) -> crate::Result<PermissionResult> {
    //     self.0
    //         .run_mobile_plugin("requestAllPermissions", ())
    //         .map_err(Into::into)
    // }
}
