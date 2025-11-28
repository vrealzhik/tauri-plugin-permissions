import SwiftRs
import Tauri
import UIKit
import CoreLocation
import UserNotifications
import CoreBluetooth

class RequestLocationArgs: Decodable {}
class RequestBluetoothArgs: Decodable {}
class RequestNotificationArgs: Decodable {}
class RequestAllPermissionsArgs: Decodable {}

class PermissionsPlugin: Plugin {
    
    @objc public func requestLocationPermission(_ invoke: Invoke) throws {
        _ = try invoke.parseArgs(RequestLocationArgs.self)
        let locationManager = CLLocationManager()
        locationManager.requestWhenInUseAuthorization()
        invoke.resolve(["granted": true])
    }
    
    @objc public func requestBluetoothPermissions(_ invoke: Invoke) throws {
        _ = try invoke.parseArgs(RequestBluetoothArgs.self)
        invoke.resolve(["granted": true])
    }
    
    @objc public func requestNotificationPermission(_ invoke: Invoke) throws {
        _ = try invoke.parseArgs(RequestNotificationArgs.self)
        let center = UNUserNotificationCenter.current()
        center.requestAuthorization(options: [.alert, .sound, .badge]) { _, _ in }
        invoke.resolve(["granted": true])      
    }
    
    @objc public func requestAllPermissions(_ invoke: Invoke) throws {
        _ = try invoke.parseArgs(RequestAllPermissionsArgs.self)
        
        let locationManager = CLLocationManager()
        locationManager.requestWhenInUseAuthorization()
        
        let center = UNUserNotificationCenter.current()
        center.requestAuthorization(options: [.alert, .sound, .badge]) { _, _ in }
        
        invoke.resolve(["granted": true])
    }
}

@_cdecl("init_plugin_permissions")
func initPlugin() -> Plugin {
    return PermissionsPlugin()
}