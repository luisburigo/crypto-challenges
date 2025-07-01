#[derive(Debug, Clone)]
pub struct Hex {
    bits: Vec<u8>,
}

impl Hex {
    pub fn new(bits: Vec<u8>) -> Self {
        Hex { bits }
    }
}

impl From<&str> for Hex {
    fn from(hex: &str) -> Self {
        let bits = hex
            .trim_start_matches("0x")
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| {
                let bytes: String = chunk.iter().collect();
                u8::from_str_radix(&bytes, 16).unwrap()
            })
            .collect();
        Hex::new(bits)
    }
}

impl From<Vec<u8>> for Hex {
    fn from(bits: Vec<u8>) -> Self {
        Hex::new(bits)
    }
}

impl Into<String> for Hex {
    fn into(self) -> String {
        let value = self
            .bits
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>();
        format!("0x{}", value.join(""))
    }
}

impl Into<Vec<u8>> for Hex {
    fn into(self) -> Vec<u8> {
        self.bits
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_hex_string_to_hex() {
        let hex_str = "0x68656c6c6f";
        let hex: Hex = hex_str.into();
        let expected = Hex::from(vec![104, 101, 108, 108, 111]);
        assert_eq!(hex, expected);
    }

    #[test]
    fn should_convert_hex_to_string() {
        let hex = Hex::from(vec![104, 101, 108, 108, 111]);
        let hex_str: String = hex.into();
        let expected = "0x68656c6c6f".to_string();
        assert_eq!(hex_str, expected);
    }

    #[test]
    fn should_convert_hex_to_vec_u8() {
        let hex = Hex::from(vec![104, 101, 108, 108, 111]);
        let vec_u8: Vec<u8> = hex.into();
        let expected = vec![104, 101, 108, 108, 111];
        assert_eq!(vec_u8, expected);
    }

    #[test]
    fn should_convert_vec_u8_to_hex() {
        let vec_u8 = vec![104, 101, 108, 108, 111];
        let hex: Hex = vec_u8.into();
        let expected = Hex::from(vec![104, 101, 108, 108, 111]);
        assert_eq!(hex, expected);
    }
}
