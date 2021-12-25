use asn1_parser::parse_asn1_file;

#[test]
fn test_enum() {
    let m = parse_asn1_file(include_str!("../resources/03-enum-OK.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}

#[test]
fn test_enum_fail() {
    parse_asn1_file(include_str!("../resources/04-enum-SE.asn1")).unwrap_err();
    parse_asn1_file(include_str!("../resources/05-enum-SE.asn1")).unwrap_err();
    parse_asn1_file(include_str!("../resources/06-enum-SE.asn1")).unwrap_err();
}
