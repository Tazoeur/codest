use data_encoding::BASE32;
use data_encoding::BASE64;

use super::DecodingTool;

pub struct Base64;

impl DecodingTool for Base64 {
    fn decode(&self, input: &str) -> Option<String> {
        match BASE64.decode(&input.chars().map(|c| c as u8).collect::<Vec<u8>>()) {
            Ok(values) => {
                self.debugok();
                Some(values.iter().map(|i| *i as char).collect::<String>())
            }
            Err(e) => {
                self.warnko(e.to_string());
                None
            }
        }
    }

    fn name(&self) -> String {
        String::from("Base 64")
    }
}

pub struct Base32;

impl DecodingTool for Base32 {
    fn decode(&self, input: &str) -> Option<String> {
        match BASE32.decode(&input.chars().map(|c| c as u8).collect::<Vec<u8>>()) {
            Ok(values) => {
                self.debugok();
                Some(values.iter().map(|i| *i as char).collect::<String>())
            }
            Err(e) => {
                self.warnko(e.to_string());
                None
            }
        }
    }

    fn name(&self) -> String {
        String::from("Base 32")
    }
}

pub struct Base58;

impl DecodingTool for Base58 {
    fn decode(&self, input: &str) -> Option<String> {
        match bs58::decode(&input).into_vec() {
            Ok(tab) => {
                self.debugok();
                Some(tab.iter().map(|i| *i as char).collect::<String>())
            }
            Err(e) => {
                self.warnko(e.to_string());
                None
            }
        }
    }

    fn name(&self) -> String {
        String::from("Base 58")
    }
}
