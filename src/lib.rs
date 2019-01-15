pub mod interface;

pub fn parse_amelia(s: String) -> String {
    interface::Amelia::go_andromeda(s)
}

pub fn gen_amelia_ed25519_key_pairing() -> String {
    interface::Amelia::gen_ed25519_key_pairing()
}