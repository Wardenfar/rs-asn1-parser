use asn1_parser::parse_asn1_file;

#[test]
fn int_ok() {
    let m = parse_asn1_file(include_str!("../resources/int-ok.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}

#[test]
fn int_err_1() {
    let e = parse_asn1_file(include_str!("../resources/int-err-1.asn1")).unwrap_err();
    insta::assert_display_snapshot!(e);
}
