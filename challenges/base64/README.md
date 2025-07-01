# Base64 Encoder

Base64 is an encoding scheme that transforms binary data into a text format using a set of 64 "safe" characters:  
`ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/`

## How It Works

1. Convert input data to binary.
2. Process the binary data in 3-byte (24-bit) chunks.
3. For each 3-byte chunk:
   - Split the 24 bits into four 6-bit groups.
   - Map each 6-bit group to a Base64 character.
   - If fewer than 3 bytes are available, pad with `=` to indicate missing bits.
4. Concatenate the resulting characters into the final encoded string.

## Encoding Steps (Example: `"Hello"`)

### Input bytes
- `H` → 01001000
- `e` → 01100101
- `l` → 01101100
- `l` → 01101100
- `o` → 01101111

### Step-by-step

#### First 3 bytes: `H`, `e`, `l` → `01001000 01100101 01101100`
- 24 bits: `010010 000110 010101 101100`
- Mapped: `S` `G` `V` `s`

#### Remaining bytes: `l`, `o` → `01101100 01101111`
- 16 bits + padding: `011011 000110 111100` → add padding
- Mapped: `b` `G` `8` `=`

### Final Output `SGVsbG8=`

## Summary

- 3 bytes → 4 Base64 characters
- Padding (`=`) is added if input length isn't a multiple of 3