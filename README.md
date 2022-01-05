# Overview
stb_image_write_rust is Rust port of stb_image_write.h, which is library to save images in BMP, JPG, PNG and TGA formats.

The porting was done using [Hebron](https://github.com/HebronFramework/Hebron).

# Crate
https://crates.io/crates/stb_image_write_rust

# Sample Code
```rust
use stb_image_write_rust::ImageWriter::ImageWriter;

fn main() {
    let mut writer = ImageWriter::new("output.jpg");
    writer.write_jpg(width, height, components, image_data, 90);
}

```

