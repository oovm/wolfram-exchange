use serde::{Serialize};
use serde_wxf::{WXFSerializer};
use wolfram_wxf::ToWolfram;


#[test]
fn test_prim() {
    let mut serializer = WXFSerializer::default();

    0usize.serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "0");

    0.0f64.serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "0`");

    true.serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "True");
}

#[test]
fn test_list() {
    let mut serializer = WXFSerializer::default();

    vec![0].serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "{0}");

    vec![vec![0],vec![1]].serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "{{0},{1}}");

    (vec![0],vec![1]).serialize(&mut serializer).unwrap();
    assert_eq!(serializer.to_wolfram_string(), "{{0},{1}}");
}

#[derive(Serialize)]
struct TestTuple(u32, Vec<&'static str>);

#[derive(Serialize)]
struct TestStruct {
    int: u32,
    seq: Vec<&'static str>,
}

#[test]
fn test_struct() {
    let mut serializer = WXFSerializer::default();
    let test = TestStruct {
        int: 1,
        seq: vec!["a", "b"],
    };
    test.serialize(&mut serializer).unwrap();
    // let expected = r#"Test["int"->1,"seq"->{"a","b"}]"#;
    assert_eq!(serializer.to_wolfram_string(), r#"<|"int"->1,"seq"->{"a","b"}|>"#);

    let test = TestTuple(
        1,
        vec!["a", "b"],
    );
    test.serialize(&mut serializer).unwrap();
    // let expected = r#"Test["int"->1,"seq"->{"a","b"}]"#;
    assert_eq!(serializer.to_wolfram_string(), r#"{1,{"a","b"}}"#);

}

//
// #[test]
// fn test_enum() {
//     #[derive(Serialize)]
//     enum E {
//         Unit,
//         Newtype(u32),
//         Tuple(u32, u32),
//         Struct { a: u32 },
//     }
//
//     let u = E::Unit;
//     let expected = r#""Unit""#;
//     assert_eq!(serialize(&u).unwrap(), expected);
//
//     let n = E::Newtype(1);
//     let expected = r#"{"Newtype":1}"#;
//     assert_eq!(serialize(&n).unwrap(), expected);
//
//     let t = E::Tuple(1, 2);
//     let expected = r#"{"Tuple":[1,2]}"#;
//     assert_eq!(serialize(&t).unwrap(), expected);
//
//     let s = E::Struct { a: 1 };
//     let expected = r#"{"Struct":{"a":1}}"#;
//     assert_eq!(serialize(&s).unwrap(), expected);
// }