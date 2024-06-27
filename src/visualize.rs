use std::io::Write;

use bytesize::ByteSize;
use log::debug;
use png::{BitDepth, ColorType, Compression, Encoder};

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type UnitResult<'a> = Result<(), BoxedError<'a>>;

/// Render the input data as a bitmap, substituting zeros for missing bytes.
fn draw_bitmap(data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let size = data.len();
    let mut bitmap = vec![0; width * height];

    for column in 0..height {
        for row in 0..width {
            let position = row + width * column;
            let pixel = if position < size { data[position] } else { 255 };

            bitmap[position] = pixel;
        }
    }

    bitmap
}

pub fn visualize<'a, W: Write>(writer: W, data: &[u8]) -> UnitResult<'a> {
    let size = data.len() as u64;
    let resolution = (size as f64).sqrt().ceil();
    let (width, height) = (resolution as u32, resolution as u32);

    debug!("\n\tSize: {}\n\tResolution: {width}x{height}", ByteSize(size));

    let mut encoder = Encoder::new(writer, width, height);

    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_compression(Compression::Best);

    let mut writer = encoder.write_header()?;
    let bitmap = draw_bitmap(data, width as usize, height as usize);

    writer.write_image_data(&bitmap)?;
    writer.finish()?;

    Ok(())
}
