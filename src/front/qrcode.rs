use bardecoder;
use iced::widget::qr_code;
use image::{self, DynamicImage};
use image::{ImageBuffer, Rgba};
//use qrc::QRCode;
//use qrc::qr_code_to;

pub struct Code {
    topic_id: String,
    //pub qr_code: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub qr_code: qr_code::Data,
}

impl Code {
    /// Convert the QRCode into a PNG representation
    pub fn new(topic_id: &str) -> Code {
        //Code {
        //    topic_id: topic_id.to_string(),
        //    image: qr_code_to!(topic_id.into(), "png", 512),
        //}
        Code {
            topic_id: topic_id.to_string(),
            qr_code: qr_code::Data::new(topic_id).expect("invalid topic_id"),
        }
    }

    /// Convert the PNG representation of the QRCode into a vector of bytes
    pub fn qr_code_to_topic_id(qr_code: DynamicImage) -> Option<String> {
        let results = bardecoder::default_decoder().decode(&qr_code);
        results.into_iter().find_map(|result| result.ok())
    }
}
