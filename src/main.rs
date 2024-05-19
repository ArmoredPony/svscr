use std::path::PathBuf;

use arboard::*;
use chrono::Utc;
use clap::*;
use home;
use image;

#[derive(Parser)]
#[command(version, about, long_about)]
/// A simple tool that allows you to quickly save a picture from the clipboard
/// to a file. Created with intent to save screenshots on Windows.
struct Args {
  /// Optional file name which gets appended to the directory. It may contain
  /// a filepath itself. If not supplied, then a timestamp is used instead
  name: Option<PathBuf>,

  #[arg(short, long, value_name = "DIR", help = "File directory", long_help)]
  /// Optional path to the directory where the image will be saved. If relative,
  /// then gets appended to user's home directory path
  directory: Option<PathBuf>,

  #[arg(
    short = 'x',
    long,
    value_name = "EXT",
    help = "File extension",
    long_help
  )]
  /// Optional file extension. Replaces the one in the file name if there is any.
  /// Default extension is '.png'.
  extension: Option<String>,
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let mut name = args
    .name
    .unwrap_or(format!("{}", Utc::now().format("%F-%H%M%S")).into());

  let path = if let Some(dir) = args.directory {
    if dir.is_absolute() {
      dir
    } else {
      home::home_dir()
        .expect("can't find home directory")
        .join(dir)
    }
  } else {
    home::home_dir().expect("can't find home directory")
  };

  if let Some(ext) = args.extension {
    name.set_extension(ext);
  } else if name.extension() == None {
    name.set_extension("png");
  }

  let path_name = path.join(name);
  let mut clipboard = Clipboard::new()?;
  let image_data = clipboard.get_image()?;
  image::save_buffer(
    path_name,
    &image_data.bytes,
    image_data.width.try_into().unwrap(),
    image_data.height.try_into().unwrap(),
    image::ExtendedColorType::Rgba8,
  )?;

  anyhow::Ok(())
}
