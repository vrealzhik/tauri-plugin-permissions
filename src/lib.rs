#![cfg(mobile)]

use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    AppHandle, Manager, Runtime,
};

pub use models::*;

// #[cfg(desktop)]
// mod desktop;
// #[cfg(mobile)]
// mod mobile;

// mod commands;
mod error;
mod models;

// use commands::*;

pub use error::{Error, Result};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.permissions";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_permissions);

pub struct Permissions<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Permissions<R> {
    pub fn get_bluetooth_permission(
        &self,
        payload: PermissionRequest,
    ) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestBluetoothPermissions", payload)
            .map_err(Into::into)
    }

    pub fn get_location_permission(
        &self,
        payload: PermissionRequest,
    ) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestLocationPermission", payload)
            .map_err(Into::into)
    }

    pub fn get_notification_permission(
        &self,
        payload: PermissionRequest,
    ) -> crate::Result<PermissionResult> {
        self.0
            .run_mobile_plugin("requestNotificationPermission", payload)
            .map_err(Into::into)
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the permissions APIs.
pub trait PermissionsExt<R: Runtime> {
    fn permissions(&self) -> &Permissions<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PermissionsExt<R> for T {
    fn permissions(&self) -> &Permissions<R> {
        self.state::<Permissions<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("permissions")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "PermissionsPlugin")?;
            #[cfg(target_os = "ios")]
            let handle = api.register_ios_plugin(init_plugin_permissions)?;
            Ok(())
        })
        .build()
}
