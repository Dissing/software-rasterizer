use std::fs::File;
use std::io::{Read, Write, BufWriter, Seek, SeekFrom, Error, ErrorKind};
use std::mem;
use std::slice;
use std::vec;
use std::ops;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        let r = (self.r as f64 * rhs) as u8;
        let g = (self.g as f64 * rhs) as u8;
        let b = (self.b as f64 * rhs) as u8;
        Color {r, g, b}
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Image {

    pub fn read(&self, x: usize, y: usize) -> Color {
        let i = (y * self.width + x) * 3;
        Color {r: self.pixels[i],
               g: self.pixels[i+1],
               b: self.pixels[i+2]}
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let i = (y * self.width + x) * 3;
            self.pixels[i] = color.b;
            self.pixels[i+1] = color.g;
            self.pixels[i+2] = color.r;
        }

    }
}

#[repr(C, packed)]
struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_entry: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_description: u8,
}

pub fn create(width: usize, height: usize) -> Image {
    let pixel_len = width * height * 3;
    Image {width, height, pixels: vec![0; pixel_len]}
}

pub fn load(name: &str) -> std::io::Result<Image> {
    let mut file = File::open(name)?;

    let mut header: TGAHeader = unsafe { mem::uninitialized()};

    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, mem::size_of::<TGAHeader>());
        file.read_exact(header_slice)?;
    }

    if header.image_type != 2 {
        return Err(Error::new(ErrorKind::InvalidData, "The TGA library currently only supports the uncompressed RGB format"));
    }

    file.seek(SeekFrom::Current(header.id_length as i64 + header.color_map_length as i64))?;

    let pixel_len = (header.width as u64 * header.height as u64 * header.pixel_depth as u64 / 8) as usize;

    let mut pixels = vec![0; pixel_len];

    file.read_exact(&mut pixels)?;

    //println!("{} {} {} {}", header.id_length, header.color_map_type, hea)

    return Ok(Image {width: header.width as usize, height: header.height as usize, pixels});
}

pub fn save(img: Image, name: &str) -> std::io::Result<()> {

    let file = File::create(name)?;
    let mut writer = BufWriter::new(file);

    let mut header =  [0; 18];

    header[2] = 2; //Image type
    header[12] = (img.width & 0x00FF) as u8;
    header[13] = ((img.width & 0xFF00) >> 8) as u8;
    header[14] = (img.height & 0x00FF) as u8;
    header[15] = ((img.height & 0xFF00) >> 8) as u8;
    header[16] = 24; //Pixel depth

    writer.write_all(&header)?;

    writer.write_all(&img.pixels)?;

    return Ok(());
}