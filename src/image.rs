use std::{path::PathBuf, fs::File, io::{BufWriter, Cursor}};

use png::{Encoder, ColorType, BitDepth, Decoder, Limits};

use crate::rgb::Rgb;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
}

#[derive(Debug)]
pub enum ImageError {
    Unsupported,
    CorruptedImage,
    Limits,
}

impl ImageError {
    pub fn from_png(err: png::DecodingError) -> ImageError {
        ImageError::CorruptedImage
    }
}

static MAGIC_BYTES: [(&[u8], ImageFormat); 5] = [
    (b"\x89PNG\r\n\x1a\n", ImageFormat::Png),
    (&[0xff, 0xd8, 0xff], ImageFormat::Jpeg),
    (b"GIF89a", ImageFormat::Gif),
    (b"GIF87a", ImageFormat::Gif),
    (b"RIFF", ImageFormat::WebP),
];

pub struct Image {
    data: Vec<Rgb>,
    dimensions: (u32, u32),
}

impl Image {
    pub fn new(data: Vec<Rgb>, dimensions: (u32, u32)) -> Image {
        Image { data, dimensions }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Rgb {
        self.data[(y * self.dimensions.0 + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb) {
        self.data[(y * self.dimensions.0 + x) as usize] = color;
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn to_u8(&self) -> Vec<u8> {
        self.data.iter().flat_map(|c| c.to_u8()).collect::<Vec<u8>>()
    }

    pub fn save(&self, file: &str) {
        let path = PathBuf::from(file);
        let file = File::create(&path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.dimensions.0, self.dimensions.1);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        match writer.write_image_data(&self.to_u8()) {
            Ok(_) => println!("Image written successfully"),
            Err(e) => println!("Error writing image: {}", e),
        }
    }
}

fn guess_format(buffer: &[u8]) -> Result<ImageFormat, ImageError> {
    for &(signature, format) in &MAGIC_BYTES {
        if buffer.starts_with(signature) {
            return Ok(format);
        }
    }
    Err(ImageError::Unsupported)
}

pub fn load_img(buf: &[u8]) -> Result<Image, ImageError> {
    let format = guess_format(buf).unwrap();
    let cursor = Cursor::new(buf);
    let mut limits = Limits::default();
    println!("> loading image");
    println!("> format: {:?}", format);
    println!("> limits: {:?}", limits.bytes);
    println!("> cursor: {:?}", cursor.position());

    match format {
        ImageFormat::Png => {
            let max_bytes = usize::try_from(limits.bytes).unwrap();
            println!("> max_bytes: {:?}", max_bytes);
            let decoder = Decoder::new_with_limits(cursor, png::Limits { bytes: max_bytes });
            let mut reader = decoder.read_info().map_err(ImageError::from_png)?;
            // let (color_type) = reader.output_color_type();

            let dimensions = reader.info().size();
            println!("> dimensions: {:?}", dimensions);
            let total_pixels = u64::from(dimensions.0) * u64::from(dimensions.1);
            println!("> total_pixels: {:?}", total_pixels);
            let bytes_per_pixel = 3;
            let total_bytes = total_pixels.saturating_mul(bytes_per_pixel) as usize;
            println!("> total_bytes: {:?}", total_bytes);

            limits.bytes -= total_bytes;

            println!("> limits: {:?}", limits.bytes);

            let mut buf = vec![Rgb::black(); total_bytes];

            while let Some(row) = reader.next_row().unwrap() {
                let row_pixels = row.data().chunks(3).map(Rgb::from).collect::<Vec<Rgb>>();
                buf.extend(row_pixels);
            }

            Ok(Image { data: buf, dimensions })
        }
        _ => Err(ImageError::Unsupported),
    }
}


