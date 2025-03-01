use image;
use image::{ImageBuffer, Rgba};
use qrc::QRCode;
use qrc::qr_code_to;
use rqrr::PreparedImage;

/// Convert the QRCode into a PNG representation
pub fn key_to_qr_code(public_key: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    qr_code_to!(public_key.into(), "png", 512)
}

/// Convert the PNG representation of the QRCode into a vector of bytes
pub fn qr_code_to_key(qr_code: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<String> {
    todo!()
    // TODO: https://docs.rs/rqrr/0.9.0/rqrr/struct.PreparedImage.html

    //    let img = image::open("tests/data/github.gif").ok()?.to_luma8();
    //    // Prepare for detection
    //    let mut img = rqrr::PreparedImage::prepare(img);
    //    // Search for grids, without decoding
    //    let grids = img.detect_grids();
    //    assert_eq!(grids.len(), 1);
    //    // Decode the grid
    //    let (meta, content) = grids[0].decode()?;
    //    Some(content)
}
