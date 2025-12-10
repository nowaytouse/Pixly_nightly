use std::io::{self, Read};
use std::process;

use anyhow::Result;

use pixly_kernel::{VideoAIPredictor, VideoEncodingPlan, VideoPredictionRequest};

fn main() {
    if let Err(err) = run() {
        eprintln!("pixly-kernel-video error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let input = buf.trim();
    if input.is_empty() {
        anyhow::bail!(
            "expected JSON VideoPredictionRequest on stdin, but input was empty",
        );
    }

    let request: VideoPredictionRequest = serde_json::from_str(input)?;

    let predictor = VideoAIPredictor::new();
    let plan: VideoEncodingPlan = predictor.predict_video_plan(&request.features, request.quality_mode);

    let json = serde_json::to_string(&plan)?;
    println!("{}", json);

    Ok(())
}
