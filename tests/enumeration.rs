use asn1_parser::module::module;

#[test]
fn test_enum() {
    let m = module(include_str!("../resources/03-enum-OK.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}
