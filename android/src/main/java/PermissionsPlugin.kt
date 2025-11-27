package com.plugin.permissions

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.ActivityCompat
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.Command
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class PermissionsPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        private const val REQUEST_CODE_LOCATION = 101
        private const val REQUEST_CODE_BLUETOOTH = 102
        private const val REQUEST_CODE_NOTIFICATIONS = 103
    }

    private var locationInvoke: Invoke? = null
    private var bluetoothInvoke: Invoke? = null
    private var notificationInvoke: Invoke? = null

    @Command
    fun requestLocationPermission(invoke: Invoke) {
        locationInvoke = invoke
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.ACCESS_FINE_LOCATION),
            REQUEST_CODE_LOCATION
        )
    }

    @Command
    fun requestBluetoothPermissions(invoke: Invoke) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) {
            invoke.resolve(JSObject().apply { put("granted", true) })
            return
        }
        bluetoothInvoke = invoke
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT
            ),
            REQUEST_CODE_BLUETOOTH
        )
    }

    @Command
    fun requestNotificationPermission(invoke: Invoke) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            invoke.resolve(JSObject().apply { put("granted", true) })
            return
        }
        notificationInvoke = invoke
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            REQUEST_CODE_NOTIFICATIONS
        )
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