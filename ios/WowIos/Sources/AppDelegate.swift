import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        WowBridge.shared.start()
        WowBridge.shared.publishDocumentsPathToRust()
        let window = UIWindow(frame: UIScreen.main.bounds)
        window.rootViewController = GameViewController()
        window.makeKeyAndVisible()
        self.window = window
        return true
    }

    func applicationDidEnterBackground(_ application: UIApplication) {
        WowBridge.shared.onLifecycle(event: .didEnterBackground)
    }

    func applicationWillEnterForeground(_ application: UIApplication) {
        WowBridge.shared.onLifecycle(event: .willEnterForeground)
    }

    func applicationDidBecomeActive(_ application: UIApplication) {
        WowBridge.shared.onLifecycle(event: .didBecomeActive)
    }

    func applicationWillResignActive(_ application: UIApplication) {
        WowBridge.shared.onLifecycle(event: .willResignActive)
    }

    func applicationWillTerminate(_ application: UIApplication) {
        WowBridge.shared.onLifecycle(event: .willTerminate)
        WowBridge.shared.shutdown()
    }
}
