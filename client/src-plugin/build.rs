const COMMANDS: &[&str] = &[
  "check_file_integrity",
  "check_resp_integrity"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
