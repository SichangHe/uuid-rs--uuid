use vstd::prelude::*;

verus! {

struct Bytes16 {
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
    b8: u8,
    b9: u8,
    b10: u8,
    b11: u8,
    b12: u8,
    b13: u8,
    b14: u8,
    b15: u8,
}

fn version4_byte(byte: u8) -> (out: u8)
    ensures
        out == (byte & 0x0f) | 0x40,
        out & 0xf0 == 0x40,
        out & 0x0f == byte & 0x0f,
{
    assert((((byte & 0x0f) | 0x40) & 0xf0) == 0x40) by (bit_vector);
    assert((((byte & 0x0f) | 0x40) & 0x0f) == (byte & 0x0f)) by (bit_vector);
    (byte & 0x0f) | 0x40
}

fn rfc4122_variant_byte(byte: u8) -> (out: u8)
    ensures
        out == (byte & 0x3f) | 0x80,
        out & 0xc0 == 0x80,
        out & 0x3f == byte & 0x3f,
{
    assert((((byte & 0x3f) | 0x80) & 0xc0) == 0x80) by (bit_vector);
    assert((((byte & 0x3f) | 0x80) & 0x3f) == (byte & 0x3f)) by (bit_vector);
    (byte & 0x3f) | 0x80
}

// @ASSUME:production_source_correspondence
// @ASSUME:caller_entropy
fn from_random_bytes_layout(input: Bytes16) -> (out: Bytes16)
    ensures
        out.b0 == input.b0,
        out.b1 == input.b1,
        out.b2 == input.b2,
        out.b3 == input.b3,
        out.b4 == input.b4,
        out.b5 == input.b5,
        out.b6 == (input.b6 & 0x0f) | 0x40,
        out.b6 & 0xf0 == 0x40,
        out.b6 & 0x0f == input.b6 & 0x0f,
        out.b7 == input.b7,
        out.b8 == (input.b8 & 0x3f) | 0x80,
        out.b8 & 0xc0 == 0x80,
        out.b8 & 0x3f == input.b8 & 0x3f,
        out.b9 == input.b9,
        out.b10 == input.b10,
        out.b11 == input.b11,
        out.b12 == input.b12,
        out.b13 == input.b13,
        out.b14 == input.b14,
        out.b15 == input.b15,
{
    let version = version4_byte(input.b6);
    let variant = rfc4122_variant_byte(input.b8);
    Bytes16 {
        b0: input.b0,
        b1: input.b1,
        b2: input.b2,
        b3: input.b3,
        b4: input.b4,
        b5: input.b5,
        b6: version,
        b7: input.b7,
        b8: variant,
        b9: input.b9,
        b10: input.b10,
        b11: input.b11,
        b12: input.b12,
        b13: input.b13,
        b14: input.b14,
        b15: input.b15,
    }
}

} // verus!
