use sherpa_rs::zipformer::ZipFormer;

fn main() {
  // Read the WAV file
  let (samples, sample_rate) =
    sherpa_rs::read_audio_file(r"E:\GitHub\ahq-ai\sherpa\models\test_wavs\0.wav").unwrap();

  let config = sherpa_rs::zipformer::ZipFormerConfig {
    encoder: r"E:\GitHub\ahq-ai\sherpa\models\encoder-epoch-99-avg-1.onnx",
    decoder: r"E:\GitHub\ahq-ai\sherpa\models\decoder-epoch-99-avg-1.onnx",
    joiner: r"E:\GitHub\ahq-ai\sherpa\models\joiner-epoch-99-avg-1.onnx",
    tokens: r"E:\GitHub\ahq-ai\sherpa\models\tokens.txt",
    ..Default::default()
  };
  let mut zipformer = ZipFormer::new(config).unwrap();
  let text = zipformer.decode(sample_rate, samples);
  println!("✅ Text: {}", text);
}
