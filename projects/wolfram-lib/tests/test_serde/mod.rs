use serde::{Deserialize, Serialize};
use wolfram_wxf::{ToWolfram, WolframFunction, WolframSerializer, WolframValue};

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Unit;

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UnitTuple(u8, u8);

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UnitStruct {
    x: u8,
    y: u8,
}

#[test]
fn fast_test() {
    let serializer = WolframSerializer::default();

    assert_eq!(true.serialize(&serializer).unwrap(), WolframValue::Boolean(true));
    assert_eq!(false.serialize(&serializer).unwrap(), WolframValue::Boolean(false));
    assert_eq!((-1i8).serialize(&serializer).unwrap(), WolframValue::Integer8(-1));
    assert_eq!(1u8.serialize(&serializer).unwrap(), WolframValue::Integer8(1));
    assert_eq!(('c').serialize(&serializer).unwrap(), WolframValue::String("c".to_string()));
    assert_eq!("str".serialize(&serializer).unwrap(), WolframValue::String("str".to_string()));
    assert_eq!(Unit.serialize(&serializer).unwrap(), WolframFunction::global("Unit", vec![]).to_wolfram());
    assert_eq!(
        UnitTuple(127, 128).serialize(&serializer).unwrap(),
        WolframFunction::global("UnitTuple", vec![127u8.serialize(&serializer).unwrap(), 128u8.serialize(&serializer).unwrap()]).to_wolfram()
    );
    assert_eq!(
        UnitStruct { x: 0, y: 0 }.serialize(&serializer).unwrap(),
        WolframFunction::global("UnitStruct", vec![WolframValue::pair("x", 127u8, false), WolframValue::pair("x", 127u8, false)]).to_wolfram()
    );
}
