use serde::{Serialize, Serializer};
use serde_wxf::serialize;

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        int: 1,
        seq: vec!["a", "b"],
    };
    // let expected = r#"Test["int"->1,"seq"->{"a","b"}]"#;
    let expected = r#"<|"int"->1,"seq"->{"a","b"}|>"#;
    assert_eq!(serialize(&test).unwrap(), expected);
}

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = r#""Unit""#;
    assert_eq!(serialize(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = r#"{"Newtype":1}"#;
    assert_eq!(serialize(&n).unwrap(), expected);

    let t = E::Tuple(1, 2);
    let expected = r#"{"Tuple":[1,2]}"#;
    assert_eq!(serialize(&t).unwrap(), expected);

    let s = E::Struct { a: 1 };
    let expected = r#"{"Struct":{"a":1}}"#;
    assert_eq!(serialize(&s).unwrap(), expected);
}