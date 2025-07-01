pub fn fixed_xor(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    value.iter().enumerate().map(|(i, &x)| x ^ key[i]).collect()
}

#[cfg(test)]
mod tests {
    use shared::Hex;
    use super::*;

    #[test]
    fn should_encode_fixed_xor_correctly() {
        let key = Hex::from("0x686974207468652062756c6c277320657965");
        let value = Hex::from("0x1c0111001f010100061a024b53535009181c");
        let expected = Hex::from("0x746865206b696420646f6e277420706c6179");

        let result: Hex = fixed_xor(value.into(), key.into()).into();
        assert_eq!(result, expected);
    }
}
