use crate::config::Config;
use image::{GenericImage, ImageBuffer, Rgb};
use qrcode::{EcLevel, QrCode, Version};

// use image::{GenericImage, ImageBuffer, Rgb};
// use std::env::args;
//
pub fn generate(conf: &Config) {
    // // Encode some data into bits.
    let code = QrCode::with_version(&conf.url, Version::Normal(3), EcLevel::L).unwrap();
    let w: u32 = code.width() as u32;
    println!("qr-code width: {:?}", w);

    let image_buf = code
        .render::<Rgb<u8>>()
        .max_dimensions(1, 1)
        .quiet_zone(false) // disable quiet zone (white border)
        .build();
    println!("image_buf size: {:?}", image_buf.dimensions());

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(conf.size, conf.size);
    image.fill(255);
    println!("image size: {:?}", image.dimensions());

    image.copy_from(&image_buf, 1, 1).unwrap();
    println!("saved path: {:?}", conf.output);
    image.save(&conf.output).unwrap();
}
