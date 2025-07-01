# Crypto Challenges

Solutions to [CryptoPals](https://cryptopals.com/), implemented in **Rust** with a focus on practical
cryptographic algorithms.

## Challenge Set 1

| Challenge                   | Status      | Path                                                 |
|-----------------------------|-------------|------------------------------------------------------|
| Base64 Encoder              | ✅ Completed | [`./challenges/base64`](./challenges/base64)         |
| Fixed XOR                   | ✅ Completed | [`./challenges/fixed_xor`](./challenges/fixed_xor)   |
| Single-byte XOR cipher      | ✅ Completed | [`./challenges/single_xor`](./challenges/single_xor) |
| Detect single-character XOR | ⏳ Pending   | -                                                    |
| Implement repeating-key XOR | ⏳ Pending   | -                                                    |
| Break repeating-key XOR     | ⏳ Pending   | -                                                    |
| AES in ECB mode             | ⏳ Pending   | -                                                    |
| Detect AES in ECB mode      | ⏳ Pending   | -                                                    |

## Goals

- Explore core concepts of modern cryptography
- Build practical Rust skills focused on security
- Solve real-world cryptanalysis challenges efficiently

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- Cargo (to run examples and tests)

## Running

```bash
cargo run --bin base64
