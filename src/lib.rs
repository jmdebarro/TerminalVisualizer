use yt_dlp::Downloader;
use std::{path::PathBuf};
use yt_dlp::client::deps::Libraries;

/// Downloads YouTube video based on given url. No Audio
pub async fn download_video(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {

    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir)
        .with_args(
            vec!["-f".into(), "worstvideo[ext=webm]/worstvideo".into()]
        )
        .build()
        .await?;

    let url = String::from(url);
    let video = downloader.fetch_video_infos(url).await?;
    let video_location: PathBuf = downloader.download_video_stream(&video, "video.mp4").await?;
    return Ok(video_location);
}

