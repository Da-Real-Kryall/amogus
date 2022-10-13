use image::gif::GifDecoder;
use image::AnimationDecoder;
use show_image::{create_window, ImageInfo, ImageView};
use soloud::*;
#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_gif: &[u8] = include_bytes!("../bettercallsaul.gif");

    let gif: GifDecoder<&[u8]> = GifDecoder::new(raw_gif)?;

    let frames = gif
        .into_frames()
        .collect_frames()?;

    let options = show_image::WindowOptions::default()
        .set_borderless(true)
        .set_size([2000, 1275])
        .set_resizable(false)
        .set_default_controls(false)
        .set_fullscreen(true);
    let window = create_window("BetterCallSaul", options)?;

    loop {
        let sl = Soloud::default()?;
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("../bettercallsaul.wav"))?;
        let fr = frames.clone();
        sl.play(&wav);
        for f in fr {
            let frame = &f.into_buffer().into_raw();
            let image = ImageView::new(ImageInfo::rgba8(2000, 1275), &frame);
            window.set_image("e", &image)?;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
