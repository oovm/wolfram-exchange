use wolfram_wxf::{ToWolfram, WolframValue};

#[test]
fn test_integer() {
    //Normal@BinarySerialize[0]
    assert_eq!(WolframValue::Integer8(0).to_bytes(), [56, 58, 67, 0]);
    assert_eq!(WolframValue::Integer8(0).to_string(), "0");
    //Normal@BinarySerialize[127]
    assert_eq!(WolframValue::Integer8(127).to_bytes(), [56, 58, 67, 127]);
    assert_eq!(WolframValue::Integer8(127).to_string(), "127");
    //Normal@BinarySerialize[-128]
    assert_eq!(WolframValue::Integer8(-128).to_bytes(), [56, 58, 67, 128]);
    assert_eq!(WolframValue::Integer8(-128).to_string(), "-128");
    //ImportByteArray@ByteArray[{56, 58, 106, 127, 0}]
    assert_eq!(WolframValue::Integer16(127).to_bytes(), [56, 58, 106, 127, 0]);
    assert_eq!(WolframValue::Integer16(127).to_string(), "127");
    //Normal@BinarySerialize[32767]
    assert_eq!(WolframValue::Integer16(32767).to_bytes(), [56, 58, 106, 255, 127]);
    assert_eq!(WolframValue::Integer16(32767).to_string(), "32767");
    //Normal@BinarySerialize[32767]
    assert_eq!(WolframValue::Integer16(-32768).to_bytes(), [56, 58, 106, 0, 128]);
    assert_eq!(WolframValue::Integer16(-32768).to_string(), "-32768");
}

#[test]
#[rustfmt::skip]
fn test_unsigned() {
    //Normal@BinarySerialize[0]
    assert_eq!(0u8.to_wolfram_bytes(), [56, 58, 67, 0]);
    //Normal@BinarySerialize[127]
    assert_eq!(127u8.to_wolfram_bytes(), [56, 58, 67, 127]);
    //Normal@BinarySerialize[128]
    assert_eq!(128u8.to_wolfram_bytes(), [56, 58, 106, 128, 0]);
    //Normal@BinarySerialize[32767]
    assert_eq!(32767u16.to_wolfram_bytes(), [56, 58, 106, 255, 127]);
    //Normal@BinarySerialize[32768]
    assert_eq!(32768u16.to_wolfram_bytes(), [56, 58, 105, 0, 128, 0, 0]);
    //Normal@BinarySerialize[2147483647]
    assert_eq!(2147483647u32.to_wolfram_bytes(), [56, 58, 105, 255, 255, 255, 127]);
    //Normal@BinarySerialize[2147483648]
    assert_eq!(2147483648u32.to_wolfram_bytes(), [56, 58, 76, 0, 0, 0, 128, 0, 0, 0, 0]);
    //Normal@BinarySerialize[9223372036854775807]
    assert_eq!(
        9223372036854775807u64.to_wolfram_bytes(),
        [56, 58, 76, 255, 255, 255, 255, 255, 255, 255, 127]
    );
    //Normal@BinarySerialize[9223372036854775808]
    assert_eq!(
        9223372036854775808u64.to_wolfram_bytes(),
        [56, 58, 73, 19, 57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56]
    );
    //Normal@BinarySerialize[170141183460469231731687303715884105727]
    assert_eq!(
        170141183460469231731687303715884105727u128.to_wolfram_bytes(),
        r"8:I'170141183460469231731687303715884105727".as_bytes()
    );
    //Normal@BinarySerialize[170141183460469231731687303715884105728]
    assert_eq!(
        170141183460469231731687303715884105728u128.to_wolfram_bytes(),
        r"8:I'170141183460469231731687303715884105728".as_bytes()
    );
}
