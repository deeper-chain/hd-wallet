use crate::transaction::{SuiRawTxIn, SuiTxOut};
use blake2b_rs::Blake2bBuilder;
use ed25519_consensus::SigningKey;
use std::convert::TryInto;
use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;

impl TraitTransactionSigner<SuiRawTxIn, SuiTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SuiRawTxIn,
    ) -> Result<SuiTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };

        let data = hex::decode(&raw_data_bytes)?;
        let mut result = [0u8; 32];
        let mut hasher = Blake2bBuilder::new(32).build();
        hasher.update(&&data);
        hasher.finalize(&mut result);

        let typed_sk = self.find_private_key(symbol, address)?;
        let sk_data = typed_sk.to_bytes();

        if sk_data.len() != 32 {
            Err(anyhow::anyhow!("privite key len not right"))?
        }

        let array: [u8; 32] = sk_data.try_into().unwrap();

        let sk = SigningKey::from(array);
        let sign_result = sk.sign(&result);

        let public_bytes = typed_sk.public_key().to_bytes();
        let mut all_sig = vec![0x0];
        all_sig.extend_from_slice(&sign_result.to_bytes());
        all_sig.extend_from_slice(&public_bytes);

        let signature = hex::encode(all_sig);
        Ok(SuiTxOut { signature })
    }
}
