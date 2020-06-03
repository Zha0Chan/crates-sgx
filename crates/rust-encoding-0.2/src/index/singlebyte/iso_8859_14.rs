// AUTOGENERATED FROM index-iso-8859-14.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-iso-8859-14.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: 2c8651cfc08b1f35b17919ee5379f2fa006af3ec809f11b3b7f470785580542b
// Date: 2014-12-19

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 7682, 7683, 163, 266, 267, 7690, 167, 7808, 169, 7810, 7691,
    7922, 173, 174, 376, 7710, 7711, 288, 289, 7744, 7745, 182, 7766, 7809,
    7767, 7811, 7776, 7923, 7812, 7813, 7777, 192, 193, 194, 195, 196, 197,
    198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 372, 209, 210, 211, 212,
    213, 214, 7786, 216, 217, 218, 219, 220, 221, 374, 223, 224, 225, 226, 227,
    228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 373, 241, 242,
    243, 244, 245, 246, 7787, 248, 249, 250, 251, 252, 253, 375, 255,
];

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

static BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
    139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
    154, 155, 156, 157, 158, 159, 160, 0, 0, 163, 0, 0, 0, 167, 0, 169, 0, 0,
    0, 173, 174, 0, 0, 0, 0, 0, 0, 0, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 193,
    194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 0,
    209, 210, 211, 212, 213, 214, 0, 216, 217, 218, 219, 220, 221, 0, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    0, 241, 242, 243, 244, 245, 246, 0, 248, 249, 250, 251, 252, 253, 0, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 165, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 178, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 240, 222, 254, 175, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 161, 162, 0, 0, 0, 0, 0, 0, 166, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 176, 177, 180, 181, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 185, 0, 0, 0, 0, 0, 0, 0, 0, 187,
    191, 0, 0, 0, 0, 0, 0, 0, 0, 215, 247, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 168, 184, 170, 186, 189, 190, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 188, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

static BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 0, 0, 32, 64, 96, 128, 160, 192, 0, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 288, 320,
    352, 0, 0, 384,
];

/// Returns the index pointer for code point `code` in this index.
#[inline]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 5) as usize;
    let offset = if offset < 248 {BACKWARD_TABLE_UPPER[offset] as usize} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 31) as usize)]
}

#[cfg(test)]
single_byte_tests!(
    mod = iso_8859_14
);