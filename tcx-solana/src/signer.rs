use crate::transaction::{SolanaRawTxIn, SolanaTxOut};
use tcx_chain::{ChainSigner, Keystore, TransactionSigner as TraitTransactionSigner};
use tcx_constants::Result;

impl TraitTransactionSigner<SolanaRawTxIn, SolanaTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &SolanaRawTxIn,
    ) -> Result<SolanaTxOut> {
        let raw_data_bytes = if tx.raw_data.starts_with("0x") {
            tx.raw_data[2..].to_string()
        } else {
            tx.raw_data.clone()
        };

        let raw_data_bytes = hex::decode(&raw_data_bytes)?;

        let sig = self.sign_recoverable_hash(&raw_data_bytes, symbol, address, None)?;

        Ok(SolanaTxOut {
            signature: hex::encode(sig),
        })
    }
}
