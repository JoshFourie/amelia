use zero_orb::{
    Proof, Z251, groth16, CoefficientPoly,
    CommonReference, Commoner, Knowledgeable,
    IntoBitsField, IntoNumField
    };

use std::path::Path;
use serde_json::to_string;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Amelia {
    pub tag: Vec<u8>,
    pub proof: Proof<Z251, Z251>,
}

impl Knowledgeable for Amelia {
    fn new(
        witness_num: Option<Vec<u8>>,
        variable_num: Option<Vec<u8>>,
        witness_bits: Option<Vec<u8>>, 
        variable_bits: Option<Vec<u8>>, 
        tag: Vec<u8>, 
        paths: [&Path; 4]
    ) -> Self {
        match (witness_num, variable_num, witness_bits, variable_bits) {
            (
                None, None, Some(witness_bits), Some(variable_bits)
            ) => {
                let crs = CommonReference::read(paths);
                let mut assignments = witness_bits.collect_bit_field().unwrap();
                assignments.append(&mut variable_bits.collect_bit_field().unwrap());
                let weights = groth16::weights(
                    std::str::from_utf8(
                        crs.code.as_slice()
                    ).unwrap(), 
                    &assignments
                ).unwrap();    
                return Self {
                    tag: tag,
                    proof: groth16::prove(
                        &crs.qap,
                        (&crs.sg1, &crs.sg2),
                        &weights
                    )
                }
            },
            (_, _, _, _) => panic!("Amelia cannot handle those assignments."),
        }        
    }
    fn check(
        self, 
        verify_num: Option<Vec<u8>>,
        verify_bits: Option<Vec<u8>>, 
        paths: [&Path; 4]
    ) -> bool {
        match (
            verify_num,
            verify_bits
        ){(
            Some(verify_num),
            Some(verify_bits)
        ) => {
            let crs = CommonReference::read(paths);
            let mut inputs = verify_num.collect_num_field().unwrap();
            inputs.append(&mut verify_bits.collect_bit_field().unwrap());
            return groth16::verify::<CoefficientPoly<Z251>, _, _, _, _>(
                (crs.sg1, crs.sg2),
                &inputs,
                self.proof
            )
        },
        (_, _,) => {panic!("Amelia cannot check those assignments.")}
        }
    }
    fn as_bits(&self) -> Vec<u8> {
        let string = to_string(&self).unwrap();
        string.as_bytes().to_vec()
    }
}

#[test]
fn test_amelias_knowledge() {
    use std::path::Path;
    for _ in 0..250 {
        let test = |input: [u8; 7], paths: [&Path; 4]| -> bool {
            Amelia::new(
                None,
                None,
                Some(vec![input[0]]),
                Some(vec![input[1], input[2]]),
                b"Athenian".to_vec(),
                paths
            ).check(
                Some(vec![input[3], input[4]]),
                Some(vec![input[5], input[6]]),
                paths
            )
        };
        let paths = [
            Path::new("src/source/amelia/amelia.zk"),
            Path::new("src/source/amelia/amelia.qap"),
            Path::new("src/source/amelia/amelia.sg1"),
            Path::new("src/source/amelia/amelia.sg2")
        ]; 
        assert_eq!(
            true, 
            test(
                [10, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [20, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [0, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [10, 5, 20, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [12, 10, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
    }
}