use std::{
    fs::{metadata, File},
    io::{BufWriter, Cursor, Read},
    path::PathBuf,
};

use png::{BitDepth, ColorType, Decoder, Encoder, Limits};

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
    pub fn from_png(_: png::DecodingError) -> ImageError {
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
    pub fn new(src: &str) -> Result<Image, ImageError> {
        let is_url = src.starts_with("http");
        println!("> loading image from {}", src);
        if is_url {
            let buffer = reqwest::blocking::get(src).unwrap().bytes().unwrap();
            return Self::load(&buffer);
        } else {
            let path = PathBuf::from(src);
            let mut file = File::open(&path).expect("> unable to open file");
            let metadata = metadata(&path).expect("> unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            file.read(&mut buffer).expect("> buffer overflow");
            return Self::load(&buffer);
        }
    }

    pub fn as_slice(&self) -> &[Rgb] {
        &self.data
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Rgb {
        self.data[(y * self.dimensions.0 + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb) {
        self.data[(y * self.dimensions.0 + x) as usize] = color;
    }

    pub fn width(&self) -> usize {
        self.dimensions.0 as usize
    }

    pub fn height(&self) -> usize {
        self.dimensions.1 as usize
    }

    pub fn into_reds(&self) -> Image {
        Image {
            data: self.data.iter().map(|p|p.as_red()).collect(),
            dimensions: self.dimensions,
        }
    }

    pub fn mean(&self) -> Image {
        Image {
            data: self.data.iter().map(|p|p.mean()).collect(),
            dimensions: self.dimensions,
        }
    }

    pub fn quantize(&self) -> Image {
        Image {
            data: self.data.iter().map(|p|p.quantize()).collect(),
            dimensions: self.dimensions,
        }
    }

    pub fn invert(&mut self) -> Image {
        Image {
            data: self.data.iter().map(|p| p.invert()).collect(),
            dimensions: self.dimensions,
        }
    }

    pub fn to_u8(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|c| c.to_u8())
            .collect::<Vec<u8>>()
    }

    pub fn save(&self, file: &str) {
        let path = PathBuf::from(file);
        let file = File::create(&path).unwrap();
        let ref mut w = BufWriter::new(file);
        println!("> creating encoder");
        let mut encoder = Encoder::new(w, self.dimensions.0, self.dimensions.1);
        println!("> setting encoder options");
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        println!("> writing image");
        match writer.write_image_data(&self.to_u8()) {
            Ok(_) => println!("> image written successfully"),
            Err(e) => println!("> error writing image: {}", e),
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

    fn load(buf: &[u8]) -> Result<Image, ImageError> {
        println!("> loading image");

        let format = Self::guess_format(buf).unwrap();
        println!("> format: {:?}", format);

        match format {
            ImageFormat::Png => {
                let cursor = Cursor::new(buf);
                let limits = Limits::default();
                let max_bytes = usize::try_from(limits.bytes).unwrap();
                println!("> max_bytes: {:?}", max_bytes);

                let mut decoder =
                    Decoder::new_with_limits(cursor, png::Limits { bytes: max_bytes });
                decoder.set_transformations(png::Transformations::EXPAND);

                let mut reader = decoder.read_info().map_err(ImageError::from_png)?;
                let (color_type, bits) = reader.output_color_type();
                println!("> color_type: {:?}", color_type);
                println!("> bits: {:?}", bits);

                let dimensions = reader.info().size();
                println!("> dimensions: {:?}", dimensions);
                let total_pixels = u64::from(dimensions.0) * u64::from(dimensions.1);
                println!("> total_pixels: {:?}", total_pixels);
                let bytes_per_pixel = 3;
                let total_bytes = total_pixels.saturating_mul(bytes_per_pixel) as usize;
                println!("> total_bytes: {:?}", total_bytes);

                let mut data = vec![];

                while let Some(row) = reader.next_row().unwrap() {
                    let row_pixels = row.data().chunks(3).map(Rgb::from).collect::<Vec<Rgb>>();
                    data.extend(row_pixels);
                }

                Ok(Image { data, dimensions })
            }
            _ => Err(ImageError::Unsupported),
        }
    }
}
