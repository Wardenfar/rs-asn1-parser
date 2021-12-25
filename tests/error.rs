use asn1_parser::parse_asn1_file;

#[test]
fn garbage() {
    parse_asn1_file(include_str!("../resources/02-garbage-NP.asn1")).unwrap_err();
}
