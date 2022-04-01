use anyhow::Result; // Automatically handle the error types
use opencv::{
    prelude::*,
    videoio,
    highgui
}; 
fn main() -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { 
            break;
        }
    }
    Ok(())
}
