use std::collections::HashMap;
use serde_json;

// match => push to vec
pub struct InboundData {
    steps: usize,
    sleep: usize,
    mindulfness: usize,
    calories: usize,
    tag: Vec<u8>,
}

pub fn parse_data(inputs: InboundData) {
    let mut data: HashMap<Vec<u8>, u8> = HashMap::new();
    match inputs.steps {
        usize => {
            match inputs.steps > 255 {
                let sliced = inputs 
            }
        }
        
        
        data.insert(
            b"steps".to_vec(),
            inputs.steps as u8
        ),
        _ => data.insert(
            b"steps".to_vec(),
            1 as u8
        ),
    };
    println!("{:?}", data);
}


#[test]
fn test_data_mappings() {
    parse_data(
        InboundData {
            steps: 2400,
            sleep: 8,
            mindulfness: 20,
            calories: 2400,
            tag: b"amelia".to_vec(),   
        }
    )
}