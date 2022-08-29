use pixels::{Error, Pixels};

pub fn redraw(pixels: &mut Pixels) -> Result<(), Error> {
    let frame = pixels.get_frame();
    let blue = [0x0, 0x0, 0xff, 0xff];
    let white = [0xff, 0xff, 0xff, 0xff];

    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&blue);
    }

    frame[0..4].copy_from_slice(&white);

    pixels.render()?;

    Ok(())
}
