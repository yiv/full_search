import Flutter
import UIKit

public class SwiftFullSearchPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "full_search", binaryMessenger: registrar.messenger())
    let instance = SwiftFullSearchPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }
  public static func dummyMethodToEnforceBundling() {
          last_error_length()
      }
}
