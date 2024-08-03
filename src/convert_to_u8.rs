use crate::defines::{BytesVector, MyImageBuffer};
use image::Rgb;
use qrcode::Color;

pub fn convert(img: &MyImageBuffer) -> BytesVector {
    assert_eq!(
        img.width(),
        img.height(),
        "Image width {:?} and height {:?} must be equal",
        img.width(),
        img.height()
    );
    let bin_size = img.width() >> 3; // 8 pixel in byte
    println!("bin size: {:?}", bin_size);

    let mut out_vec = BytesVector::new();
    add_quads(&img, &mut out_vec);
    out_vec
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
        match get_pixel(img, pos_x, y.clone()).unwrap() {
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
        add_byte(img, out_vec, x.clone(), pos_y);
    }
}

fn add_quad(img: &MyImageBuffer, out_vec: &mut BytesVector, pos_x: u32, pos_y: u32) {
    println!("add_quad: {:?} {:?}", pos_x, pos_y);
    const QUAD_POS: [[u32; 2]; 4] = [[0, 0], [8, 0], [0, 8], [8, 8]];
    for iter in QUAD_POS.into_iter() {
        add_8_bytes(
            img,
            out_vec,
            pos_x.clone() + iter[0].clone(),
            pos_y.clone() + iter[1].clone(),
        );
    }
}

fn add_quads(img: &MyImageBuffer, out_vec: &mut BytesVector) {
    let mut pos_x: u32 = 0;

    while pos_x < img.width() {
        let mut pos_y = 0;
        while pos_y < img.height() {
            add_quad(img, out_vec, pos_x.clone(), pos_y.clone());
            pos_y += 16;
        }
        pos_x += 16;
    }
}
