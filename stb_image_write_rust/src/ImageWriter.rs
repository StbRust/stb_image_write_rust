use crate::*;
use std::fs::File;
use std::io::Write;

pub struct ImageWriter {
    pub file: File,
}

impl ImageWriter {
    pub fn new(path: &str) -> ImageWriter {
        ImageWriter {
            file: File::create(path).expect("could not create file"),
        }
    }

    fn write(&mut self, data: *mut u8, size: i32) {
        let mut arr = vec![0; size as usize];

        unsafe {
            std::ptr::copy_nonoverlapping(data, arr.as_mut_ptr(), size as usize);
            self.file.write_all(&arr);
        }
    }

    pub fn write_png(&mut self, x: i32, y: i32, comp: i32, data: *const u8) {
        unsafe {
            stbi_write_png_to_func(
                ImageWriter_func,
                (self as *mut ImageWriter) as *mut u8,
                x,
                y,
                comp,
                data,
                x * comp,
            );
        }
    }

    pub fn write_jpg(&mut self, x: i32, y: i32, comp: i32, data: *const u8, quality: i32) {
        unsafe {
            stbi_write_jpg_to_func(
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

    pub fn write_bmp(&mut self, x: i32, y: i32, comp: i32, data: *const u8) {
        unsafe {
            stbi_write_bmp_to_func(
                ImageWriter_func,
                (self as *mut ImageWriter) as *mut u8,
                x,
                y,
                comp,
                data,
            );
        }
    }

    pub fn write_tga(&mut self, x: i32, y: i32, comp: i32, data: *const u8) {
        unsafe {
            stbi_write_tga_to_func(
                ImageWriter_func,
                (self as *mut ImageWriter) as *mut u8,
                x,
                y,
                comp,
                data,
            );
        }
    }
}

fn ImageWriter_func(context: *mut u8, data: *mut u8, size: i32) {
    let mut writer = context as *mut ImageWriter;
    unsafe {
        (*writer).write(data, size);
    }
}
