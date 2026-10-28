use std::{
    error::Error,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use cpal::{
    SampleFormat,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::{WhistleData, config::Config};
const RATE: u32 = 16_000;
const THRESHOLD: f32 = 0.02; // louder than this counts as speech
const END_SILENCE: f32 = 0.6; // seconds of quiet after speech, pass this and the speech is done
const MAX_WAIT: f32 = 3.0; // give up if nobody speaks
const MAX_TOTAL: f32 = 8.0; // hard cap

pub fn transcribe_audio(config: &Config) -> Result<WhistleData, Box<dyn Error>> {
    let cmd = std::process::Command::new("needle-engine")
        .args([
            "--model",
            &config.voice_model_path,
            "--audio-keywords",
            &config.keywords_path,
            "--audio-language",
            "en",
            "--audio",
            "/tmp/eery.wav",
        ])
        .output()?;
    let output = String::from_utf8_lossy(&cmd.stdout).to_lowercase();
    println!("parsed: {:?}", output);
    let output = serde_json::from_str(&output)?;
    Ok(output)
}

pub fn record() -> Result<bool, Box<dyn Error>> {
    let device = cpal::default_host()
        .default_input_device()
        .ok_or("no mic")?;
    let range = device
        .supported_input_configs()?
        .find(|c| c.sample_format() == SampleFormat::F32)
        .ok_or("no f32 input config")?;
    let config = if range.min_sample_rate() <= RATE && RATE <= range.max_sample_rate() {
        range.with_sample_rate(RATE)
    } else {
        range.with_max_sample_rate()
    };

    let ch = config.channels() as usize;
    let rate = config.sample_rate();
    let end = (END_SILENCE * rate as f32) as usize;
    let wait = (MAX_WAIT * rate as f32) as usize;
    let max = (MAX_TOTAL * rate as f32) as usize;

    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let done = Arc::new(AtomicBool::new(false));
    let spoke = Arc::new(AtomicBool::new(false));
    let (s_cb, d_cb, sp_cb) = (samples.clone(), done.clone(), spoke.clone());

    let (mut silent, mut total) = (0usize, 0usize);

    let stream = device.build_input_stream(
        config.into(),
        move |f: &[f32], _| {
            let mono: Vec<f32> = f
                .chunks(ch)
                .map(|x| x.iter().sum::<f32>() / ch as f32)
                .collect();
            let rms = (mono.iter().map(|s| s * s).sum::<f32>() / mono.len().max(1) as f32).sqrt();
            // println!("{rms:.4}"); // uncomment to tune THRESHOLD

            total += mono.len();
            if rms > THRESHOLD {
                sp_cb.store(true, Ordering::Relaxed);
                silent = 0;
            } else {
                silent += mono.len();
            }
            s_cb.lock().unwrap().extend(mono);

            let heard = sp_cb.load(Ordering::Relaxed);
            if (heard && silent >= end) || (!heard && total >= wait) || total >= max {
                d_cb.store(true, Ordering::Relaxed);
            }
        },
        |e| eprintln!("stream error: {e}"),
        None,
    )?;

    stream.play()?;
    while !done.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(20));
    }
    drop(stream);

    if !spoke.load(Ordering::Relaxed) {
        return Ok(false);
    }

    let step = (rate / RATE).max(1) as usize;
    let buf = samples.lock().unwrap();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create("/tmp/eery.wav", spec)?;
    for block in buf.chunks(step) {
        let s = block.iter().sum::<f32>() / block.len() as f32;
        w.write_sample((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
    }
    w.finalize()?;
    Ok(true)
}
