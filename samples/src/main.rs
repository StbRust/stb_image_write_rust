extern crate stb_image_write_rust;

use stb_image_write_rust::ImageWriter::ImageWriter;
use std::fs::File;
use std::io::prelude::*;

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
    }
    // Do something with it
    let outputPath = r"D:\Temp\CharacterControllerSample2.jpg";
    let mut writer = ImageWriter::new(outputPath);
    writer.write_jpg(x, y, 4, img, 90);

    unsafe {
        // Free the allocated memory
        stb_image_rust::c_runtime::free(img);
    }
}
