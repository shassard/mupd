use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let f = File::open("examples/music.ogg")?;

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(f);

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
