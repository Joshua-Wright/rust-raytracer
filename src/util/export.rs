use std::fs::File;
use std::io::{self, Write};
use raytracer::compositor::{Surface, Channel};
use std::path::Path;

pub fn to_ppm(surface: &Surface, filename: &str) -> io::Result<()> {
    let channel_max: u8 = Channel::max_value();
    let header = format!(
        "P3 {} {} {}\n", surface.width, surface.height,
        channel_max);

    let mut f = File::create(filename)?;

    f.write_all(header.as_bytes())?;
    for pixel in &surface.buffer {
        f.write_all(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_bytes())?;
    }
    Ok(())
}

pub fn to_image<P: AsRef<Path>>(surface: &Surface, path: P) -> io::Result<()> {
    let mut buf = ::image::RgbImage::new(surface.width as u32, surface.height as u32);

    for (dst_pixel, src) in buf.pixels_mut().zip(surface.iter_pixels()) {
        dst_pixel[0]=  src.r;
        dst_pixel[1]=  src.g;
        dst_pixel[2]=  src.b;
    }

    buf.save(path)
}