use integer_encoding::VarInt;

/// ```wl
/// bits = IntegerDigits[9999, 2]
/// grouped7 = Partition[Reverse[bits], UpTo[7]]
/// grouped8 = Map[Composition[PadLeft[#, 8] &, Reverse], grouped7]
/// varint = ReplacePart[grouped8, {i_, 1} /; i < Length[grouped8] :> 1]
/// Map[FromDigits[#, 2] &, varint]
///```
fn length_encoding(len: usize) -> Vec<u8> {
    let mut buf = vec![];
    let mut varint = len;
    loop {
        let next = varint & 0x7F;
        varint >>= 7;
        if varint != 0 {
            buf.push((next | 0x80) as u8)
        }
        else {
            buf.push(next as u8);
            break;
        }
    }
    return buf.to_vec();
}


#[test]
fn test_var_encoding() {
    let check = vec![0usize, 1, 64, 127, 128, 255, 256, 1024, 2048]
        .iter()
        .all(|u| u.encode_var_vec() == length_encoding(*u));
    assert!(check)
}