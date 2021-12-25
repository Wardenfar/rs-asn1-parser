use asn1_parser::parse_asn1_file;

#[test]
fn test_module_1() {
    let m = parse_asn1_file(include_str!("../resources/00-empty-OK.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}

#[test]
fn test_module_2() {
    let m = parse_asn1_file(include_str!("../resources/01-empty-OK.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}
