use pixels::{Error, Pixels};

pub fn redraw(pixels: &mut Pixels, screen: &[bool]) -> Result<(), Error> {
    let frame = pixels.get_frame();
    let black = [0x0, 0x0, 0x00, 0xff];
    let white = [0xff, 0xff, 0xff, 0xff];

    for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if screen[index] {
            pixel.copy_from_slice(&white);
        } else {
            pixel.copy_from_slice(&black);
        }
    }

    pixels.render()?;

    Ok(())
}
