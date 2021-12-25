use asn1_parser::parse_asn1_file;

#[test]
fn enum_ok() {
    let m = parse_asn1_file(include_str!("../resources/enum-ok.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}

#[test]
fn enum_err_1() {
    let e = parse_asn1_file(include_str!("../resources/enum-err-1.asn1")).unwrap_err();
    insta::assert_display_snapshot!(e)
}
