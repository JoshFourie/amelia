use zero_orb::{
    Andromeda, InterOperable, Transportable,
    Knowledge, 
    CommonReference, Common,
    FrLocal, G1Local, G2Local,
};
use serde_derive::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct Amelia {
    pub crs: String,
    pub sleep: usize,
    pub calories: usize,
}

pub fn go_andromeda(s: String) -> String {
    let amelia: Amelia = from_str(&s).expect("Amelia::go_andromeda() deserializing Amelia to struct");
    let k = Knowledge::init(
        Vec::new(),
        Vec::new(),
        vec![amelia.sleep, amelia.calories],
        Vec::new(),
        String::new(),
    );
    let crs = CommonReference::<FrLocal, G1Local, G2Local>::read(&amelia.crs);
    Andromeda::init(crs, k).go().wrap_as_str()        
}

