use crate::config::{Config, OutputMode};
use image::{GenericImage, ImageBuffer, Rgb};
use qrcode::{Color, EcLevel, QrCode, Version};
use std::fs;
use std::io::Write;

type BytesVector = Vec<u8>;
type MyImageBuffer = ImageBuffer<Rgb<u8>, BytesVector>;

const QUADS_NORMALS: [[u32; 2]; 4] = [[0, 0], [1, 0], [0, 1], [1, 1]];

pub fn generate(conf: &Config) {
    // Encode some data into bits.
    let code = QrCode::with_version(&conf.url, Version::Normal(3), EcLevel::L).unwrap();
    let w: u32 = code.width() as u32;
    println!("qr-code width: {:?}", w);
    let image = generate_image(&code, conf.size, conf.size);
    println!("saved path: {:?}", conf.output);
    match conf.mode {
        OutputMode::Image => {
            image.save(&conf.output).unwrap();
        }
        OutputMode::Binary => save_binary(&image, &conf.output),
    }
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

fn save_binary(img: &MyImageBuffer, output: &String) {
    assert_eq!(
        img.width(),
        img.height(),
        "Image {:?} must be equal {:?}",
        img.width(),
        img.height()
    );
    let bin_size = img.width() >> 3; // 8 pixel in byte
    println!("bin size: {:?}", bin_size);

    let mut out_vec = BytesVector::new();
    add_quads(&img, &mut out_vec);

    let mut file = fs::File::create(output).unwrap();
    file.write_all(&out_vec.to_vec()).unwrap();
}

fn get_pixel(img: &MyImageBuffer, x: u32, y: u32) -> Result<Color, String> {
    // println!("get pixel {:?} {:?}", x, y);
    let p = img.get_pixel(x, y);
    match p {
        Rgb([0, 0, 0]) => Ok(Color::Dark),
        Rgb([255, 255, 255]) => Ok(Color::Light),
        _ => {
            let msg = format!("Undefined color {:?}", p);
            Err(msg)
        }
    }
}

fn add_byte(img: &MyImageBuffer, out_vec: &mut BytesVector, x: u32, y: u32) {
    // println!("----- Add byte {:?} {:}", x, y);
    let mut ret_b: u8 = 0;
    for pos_x in x..(x + 8_u32) {
        match get_pixel(img, pos_x, y).unwrap() {
            Color::Light => {}
            Color::Dark => {
                ret_b |= 0b0000_0001;
            }
        }
        ret_b = ret_b << 1;
    }
    out_vec.push(ret_b)
}

fn add_8_bytes(img: &MyImageBuffer, out_vec: &mut BytesVector, x: u32, y: u32) {
    println!("---- Add 8 bytes {:?} {:}", x, y);
    for pos_y in y..(y + 8_u32) {
        add_byte(img, out_vec, x, pos_y);
    }
}

fn add_quad(img: &MyImageBuffer, out_vec: &mut BytesVector, pos_x: u32, pos_y: u32) {
    println!("add_quad: {:?} {:?}", pos_x, pos_y);
    const QUAD_POS: [[u32; 2]; 4] = [[0, 0], [8, 0], [0, 8], [8, 8]];
    for iter in QUAD_POS.into_iter() {
        add_8_bytes(img, out_vec, pos_x + iter[0], pos_y + iter[1]);
    }
}

fn add_quads(img: &MyImageBuffer, out_vec: &mut BytesVector) {
    let mut pos_x: u32 = 0;

    while pos_x < img.width() {
        let mut pos_y = 0;
        while pos_y < img.height() {
            add_quad(img, out_vec, pos_x, pos_y);
            pos_y += 16;
        }
        pos_x += 16;
    }
}
