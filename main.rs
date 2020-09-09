use std::char::decode_utf16;

// 𝄞mus<invalid>ic<invalid>
let v = [
    0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834,
];

assert_eq!(
    decode_utf16(v.iter().cloned())
        .map(|r| r.map_err(|e| e.unpaired_surrogate()))
        .collect::<Vec<_>>(),
    vec![
        Ok('𝄞'),
        Ok('m'), Ok('u'), Ok('s'),
        Err(0xDD1E),
        Ok('i'), Ok('c'),
        Err(0xD834)
    ]
);
