use rand::rngs::OsRng;
use secp256k1::rand;

use secp256k1::{PublicKey, Secp256k1, SecretKey};
pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    pub fn generate_privkey() -> SecretKey {
        let mut rng = rand::rng();
        SecretKey::new(&mut rng)
    }
    pub fn generate_pubkey() -> PublicKey {}
}
