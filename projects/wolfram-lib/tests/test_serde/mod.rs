use serde::Serialize;
use wolfram_wxf::{ToWolfram, WolframFunction, WolframSerializer, WolframValue};

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Unit;

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UnitTuple(i32, i32);

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UnitStruct {
    x: i32,
    y: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub enum UnitVariant {
    Unit,
    UnitTuple(i32, i32),
    UnitStruct { x: i32, y: i32 },
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
    assert_eq!(Some(true).serialize(&serializer).unwrap(), WolframValue::Boolean(true));
    assert_eq!(None::<bool>.serialize(&serializer).unwrap(), WolframValue::system_symbol("None"));
}

fn test_structures() {
    let serializer = WolframSerializer::default();
    assert_eq!(Unit.serialize(&serializer).unwrap(), WolframFunction::global("Unit", vec![]).to_wolfram());
    assert_eq!(
        UnitTuple(1, -1).serialize(&serializer).unwrap(),
        WolframFunction::global("UnitTuple", vec![1.serialize(&serializer).unwrap(), (-1).serialize(&serializer).unwrap()]).to_wolfram(),
    );
    assert_eq!(
        UnitStruct { x: 0, y: 0 }.serialize(&serializer).unwrap(),
        WolframFunction::global("UnitStruct", vec![WolframValue::pair("x", 0, false), WolframValue::pair("y", 0, false)]).to_wolfram(),
    );
    assert_eq!(UnitVariant::Unit.serialize(&serializer).unwrap(), WolframFunction::namespace("UnitVariant", "Unit", vec![]).to_wolfram(),);
    assert_eq!(
        UnitVariant::UnitTuple(1, -1).serialize(&serializer).unwrap(),
        WolframFunction::namespace("UnitVariant", "UnitTuple", vec![1.serialize(&serializer).unwrap(), (-1).serialize(&serializer).unwrap()])
            .to_wolfram(),
    );
    assert_eq!(
        UnitVariant::UnitStruct { x: 0, y: 0 }.serialize(&serializer).unwrap(),
        WolframFunction::namespace("UnitVariant", "UnitStruct", vec![WolframValue::pair("x", 0, false), WolframValue::pair("y", 0, false)])
            .to_wolfram(),
    );
}

#[test]
fn test2() {
    let serializer = WolframSerializer::default();
    let f = Unit.serialize(&serializer).unwrap();
    println!("{:?}", f);
}
