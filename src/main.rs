use anyhow::Result;
use candle_transformers::models::whisper as m;

use whisper::{Decoder, Task, WhichModel};

mod whisper;

fn main() -> Result<()> {
    let mut input = std::fs::File::open("audio16k.wav")?;
    let (header, data) = wav::read(&mut input)?;
    println!("loaded wav data: {header:?}");
    if header.sampling_rate != m::SAMPLE_RATE as u32 {
        anyhow::bail!("wav file must have a {} sampling rate", m::SAMPLE_RATE)
    }
    let data = data.as_sixteen().expect("expected 16 bit wav file");
    let pcm_data: Vec<_> = data[..data.len() / header.channel_count as usize]
        .iter()
        .map(|v| *v as f32 / 32768.)
        .collect();
    println!("pcm data loaded {}", pcm_data.len());

    let mut dc = Decoder::new(
        WhichModel::TinyEn,
        false,
        299792458,
        "en",
        Task::Transcribe,
        true,
        true,
    )?;

    dc.run(&pcm_data)?;
    Ok(())
}
