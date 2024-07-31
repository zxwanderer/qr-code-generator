use crate::generator::defines::MyImageBuffer;

pub fn save_image(image: &MyImageBuffer, path: &String) {
    image.save(&path).unwrap();
}
