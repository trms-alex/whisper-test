use anyhow::Result;

mod whisper;

fn main() -> Result<()> {
    whisper::whisper_main()
}
