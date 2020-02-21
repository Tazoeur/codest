use log::{debug, warn};

pub mod base;
pub mod rot;

pub trait DecodingTool {
    fn decode(&self, input: &str) -> Option<String>;
    fn name(&self) -> String;
    fn debugok(&self) {
        debug!("{} decoded successfully", self.name());
    }

    fn warnko(&self, error_message: String) {
        warn!(
            "{} encountered a decoding error : \"{}\"",
            self.name(),
            error_message
        )
    }
}

pub fn subscribed() -> Vec<Box<dyn DecodingTool>> {
    vec![
        Box::new(base::Base64),
        Box::new(base::Base32),
        Box::new(base::Base58),
        Box::new(rot::Rot13),
    ]
}
