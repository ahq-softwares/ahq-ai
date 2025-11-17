use base64::{Engine, engine::general_purpose::STANDARD};
use std::io::{self, Write};

const BEL: u8 = 0x07;

pub fn copy(data: &str) {
  let mut stdout = io::stdout();

  let data = STANDARD.encode(data);

  _ = stdout.write_all("\x1b]52;c;".as_bytes());

  _ = stdout.write_all(data.as_bytes());

  _ = stdout.write_all(&[BEL]);

  _ = stdout.flush();
}
