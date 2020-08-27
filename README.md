Wolfram Exchange Format with Rust
=================================

<img src="https://github.com/GalAster/wolfram-exchange/blob/master/projects/project-logo/Logo.png?raw=true" align="center" height="200" width="200"/>

### Design

To support format conversion, just implement the ToWolfram trait.

```rust
pub trait ToWolfram {
    fn to_wolfram(&self) -> WolframValue;
    fn to_wolfram_string(&self) -> String {self.to_wolfram().to_string()}
    fn to_wolfram_bytes(&self) -> Vec<u8> {self.to_wolfram().to_bytes()}
    fn to_wolfram_solid(&self) -> Vec<u8> {self.to_wolfram().to_compressed()}
}
```

WolframValue consists of the following legal elements:

```rust
pub enum WolframValue {
    /// Function with name, args
    Function(Box<str>, Vec<WolframValue>),
    String(Box<str>),
    Bytes(Vec<u8>),
    Symbol(Box<str>),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    BigInteger(BigInt),
    /// Do not use `f64`, because partial order cannot be defined
    Decimal64([u8; 8]),
    BigDecimal(Box<str>),
    /// Need to optimize
    PackedArray(Vec<WolframValue>),
    /// Need to optimize
    NumericArray(Vec<WolframValue>),
    /// Record with key, rule, value
    Association(BTreeMap<WolframValue, (WolframValue, WolframValue)>),
    Rule,
    RuleDelayed,
}
```


### Extension

Here are some common formats supported now:

```rust
pub enum SupportedFormat {
    JSON, //json
    TOML, //toml
    YAML, //yaml, yml
    Pickle, //pkl
}
```

Because rust cannot define traits and structs externally at the same time, supporting new formats needs to be done within this project.

See [From Traits Extension](https://github.com/GalAster/wolfram-exchange/blob/master/projects/wolfram-lib/src/utils/from_traits_extension.rs) to learn how to support new format

### Tools

- [wex](https://github.com/GalAster/wolfram-exchange-cli): A command line tool that can convert wxf format