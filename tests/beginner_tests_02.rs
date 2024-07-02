use lib_b;
use lib_b::really_complicated_code_b;

#[test]
fn test_01() {
    assert_eq!(2, really_complicated_code_b(1,1).unwrap())
}