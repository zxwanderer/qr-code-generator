use crate::generator::defines::MyImageBuffer;
use image::{GenericImage, ImageBuffer, Rgb};
use qrcode::{EcLevel, QrCode, Version};

const DEFAULT_QR_VERSION: Version = Version::Normal(3);
const DEFAULT_EC_LEVEL: EcLevel = EcLevel::L;

pub fn generate(data: &String, size: u32) -> MyImageBuffer {
    // Encode some data into bits.
    let code = QrCode::with_version(&data, DEFAULT_QR_VERSION, DEFAULT_EC_LEVEL).unwrap();
    println!("qr-code width: {:?}", code.width());
    let image = generate_image(&code, size.clone(), size);
    return image;
    // println!("saved path: {:?}", conf.output);
    // match conf.mode {
    //     OutputMode::Image => save_image(&image, &conf.output),
    //     OutputMode::Binary => save_binary(&image, &conf.output),
    // }
}

fn generate_image(code: &QrCode, w: u32, h: u32) -> MyImageBuffer {
    let image_buf: MyImageBuffer = code
        .render::<Rgb<u8>>()
        .max_dimensions(1, 1)
        .quiet_zone(false) // disable quiet zone (white border)
        .build();
    println!("image_buf size: {:?}", image_buf.dimensions());

    let mut image: MyImageBuffer = ImageBuffer::new(w, h);
    image.fill(255);
    println!("image size: {:?}", image.dimensions());

    image.copy_from(&image_buf, 1, 1).unwrap();
    image
}
