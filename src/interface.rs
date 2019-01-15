use zero_orb::{
    Andromeda, GoZero, 
    Knowledge, 
    crypto::{EdDSA, SignatureScheme},
    CommonReference, Common,
    FrLocal, G1Local, G2Local,
};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Amelia {
    crs: String,
    sleep: usize,
    calories: usize,
    output: usize,
    key_pair: String,
}

impl Amelia {
    pub fn go_andromeda(json_string: String) -> String {
        let amelia: Amelia = serde_json::from_str(&json_string)
            .expect("Amelia::go_andromeda() deserializing Amelia to struct");
        let proof_struct = Knowledge::into(
            None,
            None,
            Some(vec![amelia.sleep, amelia.calories]),
            None,
            None,
        );
        let key_pair: Box<[u8]> = serde_json::from_str(&amelia.key_pair)
            .expect("Amelia::go_andromeda() deserializing key_pair to Box<[u8>"); 
        let common_reference_string = CommonReference::<FrLocal, G1Local, G2Local>::read(&amelia.crs);
        serde_json::to_string(
            &Andromeda::into(
                common_reference_string,
                proof_struct,
                Some(vec![amelia.output]),
                key_pair,
            ).go()
        ).expect("Amelia::go_andromeda()::to_string() panicked")
    } 
    pub fn gen_ed25519_key_pairing() -> String {
        serde_json::to_string(
            &EdDSA::<String>::init_key_pair()
        ).expect("Amelia::gen_ed25519_key_pairing()::to_string() serializing EdDSA::KeyPair")
    }
}

#[test]
fn test_go_andromeda() {
    use zero_orb::{interface::{BackPack, MarkZero}, CommonReference, FrLocal, G1Local, G2Local, GtLocal};

    let amelia_serialized = serde_json::to_string(
        &Amelia {
            crs: std::fs::read_to_string("src/tests/sample.crs")
                .expect("internal_tests: reading sample.crs to String"),
            sleep: 4,
            calories: 2,
            output: 8,
            key_pair: Amelia::gen_ed25519_key_pairing(),
        }
    ).expect("internal_tests: Serializing &x to String");
    let object = Amelia::go_andromeda(amelia_serialized);
    assert!(
        serde_json::from_str::<BackPack<CommonReference<FrLocal, G1Local, G2Local>, FrLocal, G1Local, G2Local, GtLocal>>(&object)
            .expect("internal_test: serde_json::from_str::BackPack<...>::(&object) panicked when deserializing to BackPack")
            .verify()
    );
}