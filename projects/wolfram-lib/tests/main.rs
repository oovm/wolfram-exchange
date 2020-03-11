use wolfram_rs::{ToWolfram, WolframValue};

#[test]
fn test_none() {
    //Normal@BinarySerialize[System`None]
    assert_eq!(WolframValue::new_symbol("None").to_bytes(), [56, 58, 115, 4, 78, 111, 110, 101]);
    assert_eq!(WolframValue::new_symbol("None").to_string(), "None");
}

#[test]
fn test_bool() {
    //Normal@BinarySerialize[System`True]
    assert_eq!(true.to_wolfram_bytes(), [56, 58, 115, 4, 84, 114, 117, 101]);
    assert_eq!(true.to_wolfram_string(), "True");
    //Normal@BinarySerialize[System`False]
    assert_eq!(false.to_wolfram_bytes(), [56, 58, 115, 5, 70, 97, 108, 115, 101]);
    assert_eq!(false.to_wolfram_string(), "False");
}

#[test]
fn test_string() {
    //Normal@BinarySerialize["42"]
    assert_eq!("42".to_wolfram_bytes(), [56, 58, 83, 2, 52, 50]);
    assert_eq!("42".to_wolfram_string(), "\"42\"");
    //Normal@BinarySerialize["中文"]
    assert_eq!("中文".to_wolfram_bytes(), [56, 58, 83, 6, 228, 184, 173, 230, 150, 135]);
    assert_eq!("中文".to_wolfram_string(), "\"中文\"");
}
