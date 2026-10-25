use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::WhistleData;

pub fn transcribe_audio() -> Result<WhistleData, Box<dyn Error>> {
    let cmd = std::process::Command::new("engine/linux-x86_64/needle")
        .args([
            "--model",
            "./whistle.cact",
            "--audio-keywords",
            "keywords.txt",
            "--audio-language",
            "en",
            "--audio",
            "out.wav",
        ])
        .output()?;
    let output = String::from_utf8_lossy(&cmd.stdout).to_lowercase();
    println!("parsed: {:?}", output);
    let output = serde_json::from_str(&output)?;
    Ok(output)
}

pub fn record() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let mut supported_config_range = device.supported_input_configs().unwrap();
    let err_fn = |err| eprintln!("error: {}", err);
    let config = supported_config_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    let channels = config.channels();
    let spec = hound::WavSpec {
        channels,
        bits_per_sample: 32,
        sample_rate: config.sample_rate(),
        sample_format: hound::SampleFormat::Float,
    };

    let out = Arc::new(Mutex::new(Vec::new()));
    let out_clone = Arc::clone(&out);

    let stream = device.build_input_stream(
        config.into(),
        move |f: &[f32], _| {
            let data = f
                .chunks(channels as usize)
                .map(|f| f.iter().sum::<f32>() / channels as f32)
                .collect::<Vec<_>>();
            if let Ok(mut out) = out_clone.lock() {
                out.extend(data);
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    println!("Waiting for 5 seconds");
    std::thread::sleep(Duration::from_secs(5));
    println!("Finalizing.");
    drop(stream);
    let mut w = hound::WavWriter::create("out.wav", spec)?;
    let guard = out.lock().unwrap();
    for &s in guard.iter() {
        w.write_sample(s)?;
    }

    w.finalize()?;
    Ok(())
}
