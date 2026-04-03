use yt_dlp::Downloader;
use std::{path::PathBuf};
use yt_dlp::client::deps::Libraries;

/// Downloads YouTube video based on given url. No Audio
pub async fn download_video(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {

    let output_dir = PathBuf::from("output");

    // Contains info regarding the ffmpeg and yt-dlp libraries. THey are on my path
    let libraries = Libraries::new("yt-dlp".into(), "ffmpeg".into());

    // Creates the wrapper for yt_dlp with args, lib locations, and output location
    let downloader = Downloader::builder(libraries, output_dir)
        .with_args(
            vec!["-f".into(), "worstvideo[ext=webm]/worstvideo".into()]
        )
        .build()
        .await?;

    // Downloads video and stores in output path
    let url = String::from(url);
    let video = downloader.fetch_video_infos(url).await?;
    let video_location: PathBuf = downloader.download_video_stream(&video, "video.webm").await?;
    return Ok(video_location);
}

