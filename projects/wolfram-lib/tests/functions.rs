use wolfram_wxf::WolframValue;

#[test]
fn test_curry() {
    //Normal@BinarySerialize[Sin[1]]
    let v = WolframValue::Function(Box::new(WolframValue::symbol("Sin")), vec![WolframValue::Integer8(1)]);
    assert_eq!(v.to_bytes(), [56, 58, 102, 1, 115, 3, 83, 105, 110, 67, 1]);
    assert_eq!(v.to_string(), "Sin[1]");
    //Normal@BinarySerialize[Sin[1][2]]
    let v = WolframValue::Function(Box::new(v), vec![WolframValue::Integer8(2)]);
    assert_eq!(v.to_bytes(), [56, 58, 102, 1, 102, 1, 115, 3, 83, 105, 110, 67, 1, 67, 2]);
    assert_eq!(v.to_string(), "Sin[1][2]");
    //Normal@BinarySerialize[Sin[1][2][3]]
    let v = WolframValue::Function(Box::new(v), vec![WolframValue::Integer8(3)]);
    assert_eq!(v.to_bytes(), [56, 58, 102, 1, 102, 1, 102, 1, 115, 3, 83, 105, 110, 67, 1, 67, 2, 67, 3]);
    assert_eq!(v.to_string(), "Sin[1][2][3]");
}
