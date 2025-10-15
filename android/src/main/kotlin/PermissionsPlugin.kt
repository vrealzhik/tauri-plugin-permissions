package com.plugin.permissions

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class PermissionsPlugin(private val activity: Activity) : Plugin(activity) {
    companion object {
        private const val LOCATION_REQUEST_CODE = 100
        private const val NOTIFICATIONS_REQUEST_CODE = 101
        private const val BLUETOOTH_SCAN_REQUEST_CODE = 102
        private const val BLUETOOTH_CONNECT_REQUEST_CODE = 103
    }

    // Команда для проверки разрешения на местоположение (без аргументов)
    @Command
    fun checkLocationPermission(invoke: Invoke) {
        val result = JSObject()
        val status = getPermissionStatus(Manifest.permission.ACCESS_FINE_LOCATION)
        result.put("location", status)
        invoke.resolve(result)
    }

    // Команда для запроса разрешения на местоположение
    @Command
    fun requestLocationPermission(invoke: Invoke) {
        val result = JSObject()
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.ACCESS_FINE_LOCATION) != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.ACCESS_FINE_LOCATION),
                LOCATION_REQUEST_CODE
            )
            result.put("status", "request_initiated")
        } else {
            result.put("status", "already_granted")
        }
        invoke.resolve(result)
    }

    // Команда для проверки разрешения на уведомления
    @Command
    fun checkNotificationsPermission(invoke: Invoke) {
        val result = JSObject()
        val status = getPermissionStatus(Manifest.permission.POST_NOTIFICATIONS)
        result.put("notifications", status)
        invoke.resolve(result)
    }

    // Команда для запроса разрешения на уведомления
    @Command
    fun requestNotificationsPermission(invoke: Invoke) {
        val result = JSObject()
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.POST_NOTIFICATIONS),
                NOTIFICATIONS_REQUEST_CODE
            )
            result.put("status", "request_initiated")
        } else {
            result.put("status", "already_granted")
        }
        invoke.resolve(result)
    }

    // Команда для проверки разрешения на Bluetooth Scan
    @Command
    fun checkBluetoothScanPermission(invoke: Invoke) {
        val result = JSObject()
        val status = getPermissionStatus(Manifest.permission.BLUETOOTH_SCAN)
        result.put("bluetoothScan", status)
        invoke.resolve(result)
    }

    // Команда для запроса разрешения на Bluetooth Scan
    @Command
    fun requestBluetoothScanPermission(invoke: Invoke) {
        val result = JSObject()
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.BLUETOOTH_SCAN) != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.BLUETOOTH_SCAN),
                BLUETOOTH_SCAN_REQUEST_CODE
            )
            result.put("status", "request_initiated")
        } else {
            result.put("status", "already_granted")
        }
        invoke.resolve(result)
    }

    // Команда для проверки разрешения на Bluetooth Connect
    @Command
    fun checkBluetoothConnectPermission(invoke: Invoke) {
        val result = JSObject()
        val status = getPermissionStatus(Manifest.permission.BLUETOOTH_CONNECT)
        result.put("bluetoothConnect", status)
        invoke.resolve(result)
    }

    // Команда для запроса разрешения на Bluetooth Connect
    @Command
    fun requestBluetoothConnectPermission(invoke: Invoke) {
        val result = JSObject()
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.BLUETOOTH_CONNECT) != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.BLUETOOTH_CONNECT),
                BLUETOOTH_CONNECT_REQUEST_CODE
            )
            result.put("status", "request_initiated")
        } else {
            result.put("status", "already_granted")
        }
        invoke.resolve(result)
    }

    // Вспомогательная функция для получения статуса разрешения
    private fun getPermissionStatus(permission: String): String {
        return when (ContextCompat.checkSelfPermission(activity, permission)) {
            PackageManager.PERMISSION_GRANTED -> "granted"
            else -> if (ActivityCompat.shouldShowRequestPermissionRationale(activity, permission)) "prompt-with-rationale" else "prompt"
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        val result = JSObject()
        when (requestCode) {
            LOCATION_REQUEST_CODE -> {
                val granted = grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED
                result.put("location", if (granted) "granted" else "denied")
                trigger("locationPermissionResult", result)
            }
            NOTIFICATIONS_REQUEST_CODE -> {
                val granted = grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED
                result.put("notifications", if (granted) "granted" else "denied")
                trigger("notificationsPermissionResult", result)
            }
            BLUETOOTH_SCAN_REQUEST_CODE -> {
                val granted = grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED
                result.put("bluetoothScan", if (granted) "granted" else "denied")
                trigger("bluetoothScanPermissionResult", result)
            }
            BLUETOOTH_CONNECT_REQUEST_CODE -> {
                val granted = grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED
                result.put("bluetoothConnect", if (granted) "granted" else "denied")
                trigger("bluetoothConnectPermissionResult", result)
            }
        }
    }
}