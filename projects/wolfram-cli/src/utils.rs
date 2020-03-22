use wolfram_wxf::utils::parse_yaml;

pub fn test() {
    let s = "
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = parse_yaml(s);
    println!("{}", docs)
}
