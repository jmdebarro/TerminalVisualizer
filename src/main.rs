use viuer::{print_from_file, Config};
use std::error::Error;
use std::path::PathBuf;
use std::{thread, time::Duration};
use std::{fs, io, env};

mod lib;
use lib::download_video;

// Take screenshots from video and downplay frames into pixel art
// Continuously show these low res images
// Remove audio via this
// ffmpeg -i input_video.mp4 -an -c:v libx265 -crf 28 output_video.mp4
// ffmpeg: video and image tool 
// -i input | -an remove audio | -c:v copy video | libx265 -crf 28 reduce size via encoding
// ffmpeg -i dune_south.mp4 -vf "scale=80:45:flags=neighbor" frames/frame%04d.png


#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {

    // Take cmd line arg
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let url: &String = &args[1];

    // Download url provided via cmd line arg
    let video: PathBuf = download_video(url).await?;

    // Edit webm into low res with pixelated effect and many frames


    // Iterate through frames to display video
    let mut entries = fs::read_dir("./frames")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    let conf = Config {
    // Set dimensions.
    width: Some(100),
    height: Some(35),
    ..Default::default()
    };

    let frame_base = "frames";
    for path in entries {
        let frame = path.file_name().unwrap().to_str().unwrap();
        let full_path = format!("{}/{}", frame_base, frame);

        // Display `img.jpg` with dimensions 80×25 in terminal cells.
        // The image resolution will be 80×50 because each cell contains two pixels.
        print_from_file(full_path, &conf).expect("Image printing failed.");
        thread::sleep(Duration::from_millis(40));
    }
    Ok(())
}
