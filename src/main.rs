extern crate portaudio;

use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

/**
 * A sine wave implementation using wavetable synthesis, roughly taken from:
 * https://github.com/RustAudio/rust-portaudio/blob/master/examples/sine.rs
 */

fn main() {
    println!("rust-portaudio test. Sample rate: {}, Buf size: {}", SAMPLE_RATE, TABLE_SIZE);

    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        },
    }
}

fn run() -> Result<(), pa::Error> {
    // build the wavetable
    let mut sine_tbl: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE];
    for frame in 0..TABLE_SIZE {
        sine_tbl[frame] = (frame as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
    }
    let mut left_phase = 0;
    let mut right_phase = 0;

    let p = pa::PortAudio::new()?;

    let settings = p.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;

    let audio_callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        for frame in 0..frames {
            buffer[frame * 2] = sine_tbl[left_phase];
            buffer[frame * 2 + 1] = sine_tbl[right_phase];
            left_phase = (left_phase + 1) % TABLE_SIZE;
            right_phase = (right_phase + 2) % TABLE_SIZE;
        }
        pa::Continue
    };

    let mut stream = p.open_non_blocking_stream(settings, audio_callback)?;
    stream.start()?;

    println!("Playing for {} seconds", NUM_SECONDS);

    p.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Done!");

    Ok(())
}
