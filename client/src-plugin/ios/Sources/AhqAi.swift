import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class AhqAi: Plugin {

}

@_cdecl("init_plugin_ahqai")
func initPlugin() -> Plugin {
  return AhqAi()
}
