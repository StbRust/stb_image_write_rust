extern crate stb_image_write_rust;

use std::fs;
use std::fs::File;
use std::io::{prelude::*, Cursor};

use std::time::Instant;

pub struct ImageWriter {
    pub file: File,
}

impl ImageWriter {
    fn new(path: &str) -> ImageWriter {
        ImageWriter {
            file: File::create(path).expect("could not create file"),
        }
    }

    pub fn write(&mut self, data: *mut u8, size: i32) {
        let mut arr = vec![0; size as usize];

        unsafe {
            std::ptr::copy_nonoverlapping(data, arr.as_mut_ptr(), size as usize);
            self.file.write_all(&arr);
        }
    }

    unsafe fn write_png(&mut self, x: i32, y: i32, comp: i32, data: *mut u8) {
        stb_image_write_rust::stbi_write_png_to_func(
            ImageWriter_func,
            (self as *mut ImageWriter) as *mut u8,
            x,
            y,
            comp,
            data,
            x * comp,
        );
    }

    unsafe fn write_jpg(&mut self, x: i32, y: i32, comp: i32, data: *mut u8, quality: i32) {
        stb_image_write_rust::stbi_write_jpg_to_func(
            ImageWriter_func,
            (self as *mut ImageWriter) as *mut u8,
            x,
            y,
            comp,
            data,
            quality,
        );
    }
}

fn ImageWriter_func(context: *mut u8, data: *mut u8, size: i32) {
    let mut writer = context as *mut ImageWriter;
    unsafe {
        (*writer).write(data, size);
    }
}

fn main() {
    // Load file into memory
    let path = r"D:\Temp\CharacterControllerSample.jpg";
    let mut f = File::open(path).expect("file not found");
    let mut contents = vec![];
    f.read_to_end(&mut contents);

    // Load the image
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut comp: i32 = 0;
    let img: *mut u8;

    unsafe {
        img = stb_image_rust::stbi_load_from_memory(
            contents.as_mut_ptr(),
            contents.len() as i32,
            &mut x,
            &mut y,
            &mut comp,
            stb_image_rust::STBI_rgb_alpha,
        );

        // Do something with it
        let outputPath = r"D:\Temp\CharacterControllerSample2.jpg";
        let mut writer = ImageWriter::new(outputPath);
        writer.write_jpg(x, y, 4, img, 90);

        // Free the allocated memory
        stb_image_rust::c_runtime::free(img);
    }
}
