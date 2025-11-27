package com.plugin.permissions

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class PermissionPlugin(private val activity: Activity) : Plugin(activity) {

    private val handler = PermissionHandler(activity)

    private var locationInvoke: Invoke? = null
    private var bluetoothInvoke: Invoke? = null
    private var notificationInvoke: Invoke? = null

    @Command
    fun requestLocationPermission(invoke: Invoke) {
        invoke.parseArgs(RequestLocationArgs::class.java)
        if (handler.isLocationGranted()) {
            invoke.resolve(JSObject().apply { put("granted", true) })
        } else {
            locationInvoke = invoke
            handler.requestLocationPermission()
        }
    }

    @Command
    fun requestBluetoothPermissions(invoke: Invoke) {
        invoke.parseArgs(RequestBluetoothArgs::class.java)
        if (handler.isBluetoothGranted()) {
            invoke.resolve(JSObject().apply { put("granted", true) })
        } else {
            bluetoothInvoke = invoke
            handler.requestBluetoothPermissions()
        }
    }

    @Command
    fun requestNotificationPermission(invoke: Invoke) {
        invoke.parseArgs(RequestNotificationArgs::class.java)
        if (handler.isNotificationGranted()) {
            invoke.resolve(JSObject().apply { put("granted", true) })
        } else {
            notificationInvoke = invoke
            handler.requestNotificationPermission()
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        when (requestCode) {
            PermissionHandler.REQUEST_CODE_LOCATION -> {
                val granted = grantResults.getOrNull(0) == PackageManager.PERMISSION_GRANTED
                locationInvoke?.resolve(JSObject().apply { put("granted", granted) })
                locationInvoke = null
            }

            PermissionHandler.REQUEST_CODE_BLUETOOTH -> {
                val allGranted = grantResults.all { it == PackageManager.PERMISSION_GRANTED }
                bluetoothInvoke?.resolve(JSObject().apply { put("granted", allGranted) })
                bluetoothInvoke = null
            }

            PermissionHandler.REQUEST_CODE_NOTIFICATIONS -> {
                val granted = grantResults.getOrNull(0) == PackageManager.PERMISSION_GRANTED
                notificationInvoke?.resolve(JSObject().apply { put("granted", granted) })
                notificationInvoke = null
            }

            else -> super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        }
    }
}