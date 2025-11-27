package com.plugin.permissions

import android.Manifest
import android.app.Activity
import android.os.Build
import assnroid.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.Command

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
            invoke.resolve(JSObject().apply {put("granted", true)})
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
            invoke.resolve(JSObject().apply {put("granted", true)})
            return
        }
        notificationInvoke = invoke
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            REQUEST_CODE_NOTIFICATIONS
        )
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResult: IntArray) {
        when (requestCode) {
            REQUEST_CODE_LOCATION -> {
                val granted = grantResults.getOrNull(0) == PackageManager.PERMISSION_GRANTED
                locationInvoke?.resolve(JSObject().apply {put("granted", granted)})
                locationInvoke = null
            }

            REQUEST_CODE_BLUETOOTH -> {
                val granted = grantResults.getOrNull(0) == PackageManager.PERMISSION_GRANTED
                bluetoothInvoke?.resolve(JSObject().apply {put("granted", granted)})
                bluetoothInvoke = null
            }

            REQUEST_CODE_NOTIFICATIONS -> {
                val granted = grantResults.getOrNull(0) == PackageManager.PERMISSION_GRANTED
                notificationInvoke?.resolve(JSObject().apply {put("granted", granted)})
                notificationInvoke = null
            }
        }
    }


}