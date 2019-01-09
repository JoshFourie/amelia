pub mod interface;
use crate::interface::go_andromeda;

pub fn parse_amelia(s: String) -> String {
    go_andromeda(s)
}

#[test]
fn test_amelia() {
    use crate::interface::Amelia;
    use std::fs::read_to_string;
    use serde_json::to_string;

    let x = Amelia {
        crs: read_to_string("src/tests/sample.crs").expect("internal_tests: reading sample.crs to String"),
        sleep: 4,
        calories: 2400,
    };
    let s = to_string(&x).expect("internal_tests: Serializing &x to String");
    parse_amelia(s);
}