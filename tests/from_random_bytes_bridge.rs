use uuid::{Builder, Variant, Version};

const SENTINEL: [u8; 16] = [
    0xa5, 0x5a, 0x3c, 0xc3, 0x96, 0x69, 0xf0, 0x0f, 0x33, 0xcc, 0x55, 0xaa, 0x12, 0x21, 0x87, 0x78,
];

// @ASSUME:production_source_correspondence
fn assert_v4_layout(input: [u8; 16]) {
    let uuid = Builder::from_random_bytes(input).into_uuid();
    let mut expected = input;
    expected[6] = (expected[6] & 0x0f) | 0x40;
    expected[8] = (expected[8] & 0x3f) | 0x80;

    assert_eq!(*uuid.as_bytes(), expected);
    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_variant(), Variant::RFC4122);
}

#[test]
fn from_random_bytes_matches_v4_layout_corpus() {
    let mut cases = 0usize;

    for version_source in u8::MIN..=u8::MAX {
        for variant_source in u8::MIN..=u8::MAX {
            let mut input = SENTINEL;
            input[6] = version_source;
            input[8] = variant_source;
            assert_v4_layout(input);
            cases += 1;
        }
    }

    for index in 0..16 {
        for value in u8::MIN..=u8::MAX {
            let mut input = SENTINEL;
            input[index] = value;
            assert_v4_layout(input);
            cases += 1;
        }
    }

    assert_eq!(cases, 69_632);
}
