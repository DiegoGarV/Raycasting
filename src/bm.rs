use std::fs::File;
use std::io::{BufWriter, Write};
use super::color::Color;

pub fn write_bmp_file(
    file_path: &str,
    buffer: &[Color],
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    let file_size = 54 + (3 * width * height);
    let pixel_data_offset = 54;

    // BMP Header
    writer.write_all(b"BM")?; // Signature
    writer.write_all(&(file_size as u32).to_le_bytes())?; // File size
    writer.write_all(&[0u8; 4])?; // Reserved
    writer.write_all(&(pixel_data_offset as u32).to_le_bytes())?; // Pixel data offset

    // DIB Header
    writer.write_all(&(40u32).to_le_bytes())?; // DIB header size
    writer.write_all(&(width as i32).to_le_bytes())?; // Width
    writer.write_all(&(height as i32).to_le_bytes())?; // Height
    writer.write_all(&(1u16).to_le_bytes())?; // Planes
    writer.write_all(&(24u16).to_le_bytes())?; // Bits per pixel
    writer.write_all(&[0u8; 4])?; // Compression (none)
    writer.write_all(&((3 * width * height) as u32).to_le_bytes())?; // Image size
    writer.write_all(&[0u8; 16])?; // Unused

    // Pixel data
    let row_size = (3 * width + 3) & !3; // Row size, rounded up to multiple of 4 bytes
    let padding_size = row_size - 3 * width;

    for y in (0..height).rev() {
        for x in 0..width {
            let index = y * width + x;
            let color = &buffer[index];
            writer.write_all(&[color.blue(), color.green(), color.red()])?;
        }
        for _ in 0..padding_size {
            writer.write_all(&[0u8])?;
        }
    }

    writer.flush()?;
    Ok(())
}