pub fn fixed_xor(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    value.iter().enumerate().map(|(i, &x)| x ^ key[i]).collect()
}

pub fn single_xor(value: Vec<u8>, key: u8) -> Vec<u8> {
    value.iter().enumerate().map(|(i, &x)| x ^ key).collect()
}

pub fn score(value: Vec<u8>) -> u8 {
    let mut score = 0;
    let mut printable_count = 0;
    let mut space_count = 0;

    for (i, &x) in value.iter().enumerate() {
        // Characters imprevisiveis
        if x >= 32 && x <= 126 {
            printable_count += 1;
            score += 1;
        }

        // Spaces
        if x == 32 {
            space_count += 1;
            score += 3;
        }

        // Others chars
        if x < 32 && x != 10 && x != 13 {
            score = if score < 10 { 0 } else { score - 10 }
        }
    }

    let space_ratio = space_count as f64 / value.len() as f64;
    if space_ratio >= 0.10 && space_ratio <= 0.20 {
        score += 10;
    }

    let printable_ratio = printable_count as f64 / value.len() as f64;
    if printable_ratio > 0.9 {
        score += 20;
    }

    score
}

pub fn brute_force_single_xor(value: Vec<u8>) -> Option<(Vec<u8>, u8)> {
    let mut results = Vec::new();

    for i in 0..255 {
        let encrypted = single_xor(value.to_vec(), i.into());
        let encrypted_score = score(encrypted.clone());
        results.push((encrypted.clone(), encrypted_score, i));
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results
        .get(0)
        .map(|(vec, _score, key)| (vec.clone(), key.clone()))
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let value = bytes
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>();

    format!("0x{}", value.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_fixed_xor_correctly() {
        let key = hex_to_bytes("0x686974207468652062756c6c277320657965");
        let value = hex_to_bytes("0x1c0111001f010100061a024b53535009181c");
        let expected = hex_to_bytes("0x746865206b696420646f6e277420706c6179");

        let result = fixed_xor(value, key);
        assert_eq!(result, expected);
    }

    #[test]
    fn should_encode_single_xor_correctly() {
        let key: u8 = 88;
        let value = hex_to_bytes("0x1c0111001f010100061a024b53535009181c");
        let expected = hex_to_bytes("0x44594958475959585e425a130b0b08514044");

        let asd = bytes_to_hex(expected.as_slice());

        let result = single_xor(value, key);
        assert_eq!(result, expected);
    }

    #[test]
    fn should_brute_force_key() {
        let key: u8 = 88;
        let value = "XCooking MC's like a pound of bacon";
        let encoded = single_xor(value.as_bytes().to_vec(), key);
        let decoded = brute_force_single_xor(encoded);
        assert_eq!(decoded, Some((value.as_bytes().to_vec(), key)));
    }
}
