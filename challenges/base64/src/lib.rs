const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64(data: &[u8]) -> String {
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let mut buffer = [0u8; 3];

        // Copy the available bytes
        for (i, &byte) in chunk.iter().enumerate() {
            buffer[i] = byte;
        }

        // Convert 3 bytes in 4 groups of 6 bits
        let combined = ((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32);

        // Get 4 groups of 6 bits
        let indexes = [
            ((combined >> 18) & 0x3F) as usize,
            ((combined >> 12) & 0x3F) as usize,
            ((combined >> 6) & 0x3F) as usize,
            (combined & 0x3F) as usize,
        ];

        // Convert the chars to Base64
        match chunk.len() {
            // No need to add padding
            3 => {
                for &index in indexes.iter() {
                    result.push(BASE64_CHARS[index] as char);
                }
            }
            // Add padding in the index 3
            2 => {
                result.push(BASE64_CHARS[indexes[0]] as char);
                result.push(BASE64_CHARS[indexes[1]] as char);
                result.push(BASE64_CHARS[indexes[2]] as char);
                result.push('=');
            }
            // Add padding in the index 2 and 3
            1 => {
                result.push(BASE64_CHARS[indexes[0]] as char);
                result.push(BASE64_CHARS[indexes[1]] as char);
                result.push('=');
                result.push('=');
            }
            _ => unreachable!(),
        }
    }

    result
}

pub fn str_to_base64(data: &str) -> String {
    base64(data.as_bytes())
}

pub fn hex_to_base64(data: &str) -> String {
    let hex: Vec<u8> = data.trim_start_matches("0x")
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| {
            let bytes: String = chunk.iter().collect();
            u8::from_str_radix(&bytes, 16).unwrap()
        })
        .collect();
    
    base64(&hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_str_to_base64_correctly() {
        let message = "Encoded";
        let expected = "RW5jb2RlZA==";
        let encoded = str_to_base64(message);
        assert_eq!(encoded, expected);
    }
    
    #[test]
    fn should_encode_hex_to_base64_correctly() {
        let message = "0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let encoded = hex_to_base64(message);
        assert_eq!(encoded, expected);
    }
}
