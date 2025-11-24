use std::sync::mpsc::channel;

use sherpac::{
  model::{OnlineModelConfig, Provider},
  recognizer::{DecodingMethod, OnlineRecognizer, OnlineRecognizerConfig},
  stream::online::OnlineStream,
  transducer::OnlineTransducerModelConfig,
};

use cpal::{
  default_host,
  traits::{DeviceTrait, HostTrait, StreamTrait},
};

fn main() {
  let transducer = OnlineTransducerModelConfig::new(
    r"E:\GitHub\ahq-ai\sherpa\sherpac\models\encoder-epoch-99-avg-1.onnx",
    r"E:\GitHub\ahq-ai\sherpa\sherpac\models\decoder-epoch-99-avg-1.onnx",
    r"E:\GitHub\ahq-ai\sherpa\sherpac\models\joiner-epoch-99-avg-1.onnx",
  )
  .unwrap();

  let model = OnlineModelConfig::new(
    transducer,
    Provider::Cpu,
    r"E:\GitHub\ahq-ai\sherpa\sherpac\models\tokens.txt",
  )
  .unwrap();

  let conf = OnlineRecognizerConfig::new(model, DecodingMethod::GreedySearch).unwrap();
  let recog = OnlineRecognizer::new(conf).unwrap();

  let mic = default_host();
  let device = mic.default_input_device().unwrap();

  let mut supported_configs_range = device
    .supported_input_configs()
    .expect("error while querying configs");

  let supported_config = supported_configs_range
    .next()
    .expect("no supported config?!")
    .with_max_sample_rate();

  let sample_rate = supported_config.sample_rate().0;

  let stream = OnlineStream::new(recog.clone(), sample_rate as i32).unwrap();

  let (tx, rx) = channel();

  let result = stream.decode(rx).unwrap();

  let micstream = device
    .build_input_stream(
      &supported_config.config(),
      move |data: &[f32], _: &cpal::InputCallbackInfo| {
        _ = tx.send(data.into());
      },
      move |_| {},
      None, // None=blocking, Some(Duration)=timeout
    )
    .unwrap();

  micstream.play().unwrap();

  let mut old = String::new();
  while let Ok(data) = result.recv() {
    let data = data.text_lossy().unwrap().into_owned();

    if &data != &old {
      println!("{}", data);
      old = data;
    }
  }
}
