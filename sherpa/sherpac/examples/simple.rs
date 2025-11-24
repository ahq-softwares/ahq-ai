use std::{
  ffi::{CStr, CString},
  mem,
};

use sherpac::raw::{
  SherpaOnnxAcceptWaveformOffline, SherpaOnnxCreateOfflineRecognizer,
  SherpaOnnxCreateOfflineStream, SherpaOnnxDecodeOfflineStream, SherpaOnnxDestroyOfflineRecognizer,
  SherpaOnnxDestroyOfflineRecognizerResult, SherpaOnnxDestroyOfflineStream, SherpaOnnxFreeWave,
  SherpaOnnxGetOfflineStreamResult, SherpaOnnxOfflineModelConfig,
  SherpaOnnxOfflineRecognizerConfig, SherpaOnnxOfflineTransducerModelConfig, SherpaOnnxReadWave,
};

fn main() {
  let wav_filename =
    CString::new(r"E:\GitHub\ahq-ai\sherpa\sherpac\models\test_wavs\1.wav").unwrap();

  let joiner =
    CString::new(r"E:\GitHub\ahq-ai\sherpa\sherpac\models\joiner-epoch-99-avg-1.onnx").unwrap();
  let decoder =
    CString::new(r"E:\GitHub\ahq-ai\sherpa\sherpac\models\decoder-epoch-99-avg-1.onnx").unwrap();
  let encoder =
    CString::new(r"E:\GitHub\ahq-ai\sherpa\sherpac\models\encoder-epoch-99-avg-1.onnx").unwrap();
  let tokens = CString::new(r"E:\GitHub\ahq-ai\sherpa\sherpac\models\tokens.txt").unwrap();

  let provider = CString::new("cpu").unwrap();

  let search = CString::new("greedy_search").unwrap();

  unsafe {
    let wave = SherpaOnnxReadWave(wav_filename.as_ptr());

    if wave.is_null() {
      panic!("Impossible NULL pointer");
    }

    let mut config: SherpaOnnxOfflineTransducerModelConfig = mem::zeroed();

    config.encoder = encoder.as_ptr();
    config.decoder = decoder.as_ptr();
    config.joiner = joiner.as_ptr();

    let mut offline_model_config: SherpaOnnxOfflineModelConfig = mem::zeroed();

    offline_model_config.debug = 0;
    offline_model_config.num_threads = 4;

    offline_model_config.provider = provider.as_ptr();
    offline_model_config.tokens = tokens.as_ptr();
    offline_model_config.transducer = config;

    let mut recognizer_config: SherpaOnnxOfflineRecognizerConfig = mem::zeroed();

    recognizer_config.decoding_method = search.as_ptr();
    recognizer_config.model_config = offline_model_config;

    let recognizer = SherpaOnnxCreateOfflineRecognizer(&recognizer_config);
    if recognizer.is_null() {
      panic!("Impossible NULL pointer");
    }

    println!("Starting Streaming Works");

    let stream = SherpaOnnxCreateOfflineStream(recognizer);

    if stream.is_null() {
      panic!("Impossible NULL pointer while making Stream");
    }

    println!("Created Stream");

    SherpaOnnxAcceptWaveformOffline(
      stream,
      (&*wave).sample_rate,
      (&*wave).samples,
      (&*wave).num_samples,
    );

    println!("Decoding");

    SherpaOnnxDecodeOfflineStream(recognizer, stream);

    println!("Getting Stream Result");

    let result = SherpaOnnxGetOfflineStreamResult(stream);

    println!("Got Stream Result");

    if result.is_null() {
      panic!("Impossible NULL pointer while making Result");
    }

    let txt = CStr::from_ptr((&*result).text);

    let data = txt.to_str().unwrap();

    println!("{data}");

    SherpaOnnxDestroyOfflineRecognizerResult(result);
    SherpaOnnxDestroyOfflineStream(stream);
    SherpaOnnxDestroyOfflineRecognizer(recognizer);
    SherpaOnnxFreeWave(wave);
  }
}
