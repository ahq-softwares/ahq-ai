use std::{fs, process::Command};

use inquire::Confirm;

fn exec(cmd: &str, cwd: Option<&str>) {
  let mut data = cmd.split(" ");

  let bin = data.next().unwrap();
  let args = data.collect::<Vec<_>>();

  let mut cmd = Command::new(bin);
  
  cmd.args(args);
  
  if let Some(cwd) = cwd {
    cmd.current_dir(cwd);
  }

  cmd.spawn()
    .unwrap()
    .wait()
    .unwrap()
    .exit_ok()
    .unwrap();
}

pub fn compile() {
  println!(r"
      ___    __  ______      ______            _____      
   /   |  / / / / __ \    / ____/___  ____  / __(_)___ _
  / /| | / /_/ / / / /   / /   / __ \/ __ \/ /_/ / __ `/
 / ___ |/ __  / /_/ /   / /___/ /_/ / / / / __/ / /_/ / 
/_/  |_/_/ /_/\___\_\   \____/\____/_/ /_/_/ /_/\__, /  
                                               /____/   
  ");

  #[cfg(target_os = "macos")]
  println!("> On macos, Metal is automatically selected, if available");

  let mut cmd = "cmake -B build -DCMAKE_BUILD_TYPE=release".to_string();

  if Confirm::new("Do you want curl support (required libcurl to be installed)?")
    .with_default(false)
    .prompt_skippable()
    .unwrap()
    .unwrap_or_default() {
    cmd.push_str(" -DLLAMA_CURL=ON");
  } else {
    cmd.push_str(" -DLLAMA_CURL=OFF");
  }

  if Confirm::new("Do you want CUDA support (associated toolkit should be installed)?")
    .with_default(false)
    .prompt_skippable()
    .unwrap()
    .unwrap_or_default() {
    cmd.push_str(" -DGGML_CUDA=ON");
  }

  if Confirm::new("Do you want CANN support (associated toolkit should be installed)?")
    .with_default(false)
    .prompt_skippable()
    .unwrap()
    .unwrap_or_default() {
    cmd.push_str(" -DGGML_CANN=ON");
  }

  #[cfg(target_os = "linux")]
  if Confirm::new("Do you want OpenBLAS support (associated toolkit should be installed)?")
    .with_default(false)
    .prompt_skippable()
    .unwrap()
    .unwrap_or_default() {
      cmd.push_str(" -DGGML_BLAS=ON -DGGML_BLAS_VENDOR=OpenBLAS");
    }

  let exists = fs::exists("./llama.cpp").unwrap();

  let clone;

  if exists {
    clone = Confirm::new("Do you want to overwrite llama.cpp directory?")
      .with_default(false)
      .prompt_skippable()
      .unwrap()
      .unwrap_or_default();

    if clone {
      fs::remove_dir_all("./llama.cpp").unwrap();
    }
  } else {
    clone = true;
  }
  
  if clone {
    exec("git clone https://github.com/ggml-org/llama.cpp.git", None);
  }

  exec(&cmd, Some("llama.cpp"));

  exec("cmake --build build --config Release -j 8", Some("llama.cpp"));

  _ = fs::remove_dir_all("./llama");
  copy();
}

fn copy() {
  fs::create_dir_all("./llama").unwrap();

  let mut path = "./llama.cpp/build/bin/Release";
  let release = fs::exists("./llama.cpp/build/bin/Release").unwrap();

  if !release {
    path = "./llama.cpp/build/bin";
  }

  for entry in fs::read_dir(path).unwrap().into_iter().map(|x| x.unwrap()) {
    let name = entry.file_name().into_string().unwrap();

    fs::copy(entry.path(), format!("./llama/{name}")).unwrap();
  }
}
