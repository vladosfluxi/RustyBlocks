use rand::rngs::OsRng;
use ripemd::{Digest, Ripemd160};
use secp256k1::rand;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::Sha256;

pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    pub fn generate_privkey() -> SecretKey {
        let mut rng = rand::rng();
        SecretKey::new(&mut rng)
    }
    pub fn generate_pubkey(private_key: &SecretKey) -> PublicKey {
        let secp = Secp256k1::new();
        PublicKey::from_secret_key(&secp, &private_key)
    }

    pub fn new() -> Self {
        let private_key = Wallet::generate_privkey();

        Wallet {
            private_key,
            public_key: Wallet::generate_pubkey(&private_key),
        }
    }
    pub fn generate_address(&self) -> [u8; 20] {
        let pubkey_bytes = &self.public_key.serialize();
        let sha = Sha256::digest(&pubkey_bytes);
        let address = Ripemd160::digest(&sha);
        address.into()
    }
}
