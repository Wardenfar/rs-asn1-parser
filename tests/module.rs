use asn1_parser::parse_asn1_file;

#[test]
fn empty_ok() {
    let m = parse_asn1_file(include_str!("../resources/empty-ok.asn1")).unwrap();
    insta::assert_json_snapshot!(m);
}

#[test]
fn garbage_err() {
    let e = parse_asn1_file(include_str!("../resources/garbage-err.asn1")).unwrap_err();
    insta::assert_display_snapshot!(e)
}
