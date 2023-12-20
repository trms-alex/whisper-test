use anyhow::Result;
use whisper::{whisper_main, Task, WhichModel};

mod whisper;

fn main() -> Result<()> {
    whisper_main(
        false,
        WhichModel::TinyEn,
        "audio16k.wav",
        299792458,
        false,
        "en",
        Task::Translate,
        true,
    )
}
