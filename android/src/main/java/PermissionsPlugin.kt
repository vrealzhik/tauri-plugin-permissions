package com.plugin.permissions

import android.Manifest
import android.app.Activity
import android.os.Build
import androidx.core.app.ActivityCompat
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.Command

@TauriPlugin
class PermissionsPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun requestLocationPermission(invoke: Invoke) {
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.ACCESS_FINE_LOCATION),
            101
        )
        invoke.resolve(JSObject().apply { put("status", "requested") })
    }

    @Command
    fun requestBluetoothPermissions(invoke: Invoke) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT
                ),
                102
            )
        }
        invoke.resolve(JSObject().apply { put("status", "requested") })
    }

    @Command
    fun requestNotificationPermission(invoke: Invoke) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.POST_NOTIFICATIONS),
                103
            )
        }
        invoke.resolve(JSObject().apply { put("status", "requested") })
    }

    @Command
    fun requestAllPermissions(invoke: Invoke) {
        val permissions = mutableListOf<String>()

        permissions.add(Manifest.permission.ACCESS_FINE_LOCATION)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            permissions.add(Manifest.permission.BLUETOOTH_SCAN)
            permissions.add(Manifest.permission.BLUETOOTH_CONNECT)
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            permissions.add(Manifest.permission.POST_NOTIFICATIONS)
        }

        ActivityCompat.requestPermissions(
            activity,
            permissions.toTypedArray(),
            200
        )

        invoke.resolve(JSObject().apply { put("status", "requested") })
    }
}