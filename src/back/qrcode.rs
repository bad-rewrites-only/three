use bardecoder;
use image::{self, DynamicImage};
use image::{ImageBuffer, Rgba};
use qrc::QRCode;
use qrc::qr_code_to;

/// Convert the QRCode into a PNG representation
pub fn key_to_qr_code(public_key: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    qr_code_to!(public_key.into(), "png", 512)
}

/// Convert the PNG representation of the QRCode into a vector of bytes
pub fn qr_code_to_key(qr_code: DynamicImage) -> Option<String> {
    let results = bardecoder::default_decoder().decode(&qr_code);
    results.into_iter().find_map(|result| result.ok())
}
