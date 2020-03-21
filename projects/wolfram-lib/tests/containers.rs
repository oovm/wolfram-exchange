use std::collections::{BTreeMap, HashMap};
use wolfram_wxf::ToWolfram;

#[test]
fn test_list() {
    //Normal@BinarySerialize[{0}]
    assert_eq!(vec![0u8].to_wolfram_bytes(), [56, 58, 102, 1, 115, 4, 76, 105, 115, 116, 67, 0]);
    assert_eq!(vec![0u8].to_wolfram_string(), "{0}");
    //Normal@BinarySerialize[{"0"}]
    assert_eq!(vec!["0"].to_wolfram_bytes(), [56, 58, 102, 1, 115, 4, 76, 105, 115, 116, 83, 1, 48]);
    assert_eq!(vec!["0"].to_wolfram_string(), "{\"0\"}");
}

#[test]
fn test_dict() {
    //Normal@BinarySerialize[<|1 -> 2|>]
    let mut dict = BTreeMap::new();
    dict.insert(1u8, 2u8);
    assert_eq!(dict.to_wolfram_bytes(), [56, 58, 65, 1, 45, 67, 1, 67, 2]);
    assert_eq!(dict.to_wolfram_string(), "<|1->2|>");
    //Normal@BinarySerialize[<|1 -> 2|>]
    let mut dict = HashMap::new();
    dict.insert(1u8, 2u8);
    assert_eq!(dict.to_wolfram_bytes(), [56, 58, 65, 1, 45, 67, 1, 67, 2]);
    assert_eq!(dict.to_wolfram_string(), "<|1->2|>");
}
