const LIBS: [&'static str; 16] = [
  "cargs",
  "cppinyin_core",
  "espeak-ng",
  "kaldi-decoder-core",
  "kaldi-native-fbank-core",
  "kissfft-float",
  "onnxruntime",
  "piper_phonemize",
  "sherpa-onnx-c-api",
  "sherpa-onnx-core",
  "sherpa-onnx-fst",
  "sherpa-onnx-fstfar",
  "sherpa-onnx-kaldifst-core",
  "sherpa-onnx-portaudio_static",
  "ssentencepiece_core",
  "ucd",
];

#[cfg(windows)]
const SYSTEM_DLLS: [&'static str; 5] = ["ws2_32", "dbghelp", "userenv", "kernel32", "advapi32"];

#[cfg(not(windows))]
const SYSTEM_DLLS: [&'static str; 4] = ["m", "dl", "log", "pthread"];

fn main() {
  println!("cargo:rustc-link-search=native=./lib");

  for item in LIBS {
    println!("cargo:rustc-link-lib=static={item}");
  }

  for item in SYSTEM_DLLS {
    println!("cargo:rustc-link-lib=dylib={item}");
  }
}
