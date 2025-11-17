use std::time::SystemTime;

use chalk_rs::Chalk;
use log::Level;

pub fn setup() {
  let mut chalk = Chalk::new();
  let info = chalk.blue().string(&"INFO").leak();
  let warn = chalk.yellow().string(&"WARN").leak();
  let err = chalk.red().bold().string(&"ERRR").leak();

  let fe = fern::Dispatch::new().format(|out, message, record| {
    let mut chalk = Chalk::new();

    let (level, msg) = match record.level() {
      Level::Trace => ("TRCE", message.to_string()),
      Level::Debug => ("DEBG", message.to_string()),
      Level::Info => (&*info, chalk.blue().string(&message)),
      Level::Warn => (&*warn, chalk.yellow().string(&message)),
      Level::Error => (&*err, chalk.red().bold().string(&message)),
    };

    let target_str = record.target();

    let target = if target_str.starts_with("ahqai_server::") {
      "".into()
    } else {
      chalk.reset_style().dim().string(&format!("({target_str})"))
    };

    out.finish(format_args!(
      "[{} {level}] {msg} {target}",
      humantime::format_rfc3339_seconds(SystemTime::now()),
    ))
  });
  #[cfg(debug_assertions)]
  let fe = fe.level(log::LevelFilter::Debug);
  #[cfg(not(debug_assertions))]
  let fe = fe.level(log::LevelFilter::Info);

  fe.chain(std::io::stdout()).apply().unwrap();
}
