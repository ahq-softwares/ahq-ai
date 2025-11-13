#![feature(exit_status_error)]

pub mod compile;

#[cfg(all(any(windows, target_os = "linux"), target_arch = "x86_64"))]
pub mod download;

use inquire::Confirm;
use std::process;

#[tokio::main]
#[cfg(not(all(any(windows, target_os = "linux"), target_arch = "x86_64")))]
async fn main() {
  println!("We must compile llama.cpp");

  let resp = Confirm::new("Do you have C/C++ tools like `cmake` and `git` installed?")
    .with_default(true)
    .prompt()
    .unwrap_or_default();

  if !resp {
    println!("Please install the tools and try again");
    process::exit(0);
  }

  compile::compile();
}

#[tokio::main]
#[cfg(all(any(windows, target_os = "linux"), target_arch = "x86_64"))]
async fn main() {
  use inquire::Select;

  println!(r"
      ___    __  ______      ___    ____
   /   |  / / / / __ \    /   |  /  _/
  / /| | / /_/ / / / /   / /| |  / /  
 / ___ |/ __  / /_/ /   / ___ |_/ /   
/_/  |_/_/ /_/\___\_\  /_/  |_/___/   
                                    
  ");

  let compile = "Compile from source";
  let download = "Download prebuilt";

  let select = Select::new("How do you want to get llama.cpp", vec![download, compile])
    .prompt()
    .expect("Must respond");

  if select == download {
    use std::io::Cursor;

    let (tag, assets) = download::get_platform_assets().await;

    println!("\n> Found Release {tag}\n");

    let cpu = Confirm::new(
      "Do you know about the cpu features of this computer (eg, AVX1, AVX2, AMX-Int8 etc)?",
    )
    .with_default(false)
    .prompt()
    .unwrap_or_default();

    let name = if cpu {
      Select::new("Select the build based on your features", assets.iter().map(|x| &x.name as &str).collect()).prompt().expect("Must select the build").to_string()
    } else {
      #[cfg(windows)]
      let asset_name = "llama-cpp-windows-x64-noavx.zip";
      #[cfg(unix)]
      let asset_name = "llama-cpp-linux-x64-noavx.zip";

      asset_name.to_string()
    };

    let asset = assets.into_iter().find(|x| &x.name == &name).unwrap();

    let file = Cursor::new(download::dwnl(&asset.browser_download_url).await);
    
    let mut writer = zip::read::ZipArchive::new(file).unwrap();
    writer.extract("./llama").unwrap();
    drop(writer);
  } else {
    println!("> Please wait while we compile using cmake...");

    let resp = Confirm::new("Do you have C/C++ tools like `cmake` and `git` installed?")
      .with_default(true)
      .prompt()
      .unwrap_or_default();

    if !resp {
      println!("> Please install the tools and try again");
      process::exit(0);
    }

    compile::compile();
  }

  println!("> Done. llama.cpp has been instantiated in the /llama/ folder");
}
