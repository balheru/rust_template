use lib_a;
use lib_a::really_complicated_code_a;

#[test]
fn test_01() {
    assert_eq!(2, really_complicated_code_a(1,1).unwrap())
}