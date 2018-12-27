mod amelia;

#[test]
fn test_comparator() {
    use zero_orb::code_gen::comparator;
    use std::path::Path;
    
    comparator::build(
        3, 
        Path::new("src/source/comparator.zk")
    );
}