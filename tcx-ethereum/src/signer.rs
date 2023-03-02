use crate::transaction::{EthereumTxIn, EthereumTxOut};
use crate::{chain_id_from_network, Error};
use ethereum_tx_sign::{LegacyTransaction, Transaction};
use ethereum_types::{H160, H256, U256};
use std::convert::TryFrom;
use std::str::FromStr;

use tcx_chain::{Keystore, Result, TransactionSigner};

impl TryFrom<&EthereumTxIn> for LegacyTransaction {
    type Error = crate::Error;

    fn try_from(input: &EthereumTxIn) -> core::result::Result<Self, Self::Error> {
        let nonce = u128::from_str(input.nonce.as_str()).map_err(|_| Error::InvalidNonce)?;
        let to = if input.to.len() > 0 {
            Some(
                H160::from_str(input.to.as_str())
                    .map_err(|_| Error::InvalidTo)?
                    .to_fixed_bytes(),
            )
        } else {
            None
        };
        let value = u128::from_str(input.value.as_str()).map_err(|_| Error::InvalidValue)?;
        let gas_price =
            u128::from_str(input.gas_price.as_str()).map_err(|_| Error::InvalidGasPrice)?;
        let gas = u128::from_str(input.gas.as_str()).map_err(|_| Error::InvalidGas)?;
        let data = hex::decode(input.data.clone()).map_err(|_| Error::InvalidData)?;
        let chain =
            chain_id_from_network(input.network.as_str()).map_err(|_| Error::InvalidChainId)?;
        Ok(LegacyTransaction {
            nonce,
            to,
            value,
            gas_price,
            gas,
            data,
            chain,
        })
    }
}

impl TransactionSigner<EthereumTxIn, EthereumTxOut> for Keystore {
    fn sign_transaction(
        &mut self,
        symbol: &str,
        address: &str,
        tx: &EthereumTxIn,
    ) -> Result<EthereumTxOut> {
        let unsigned_tx = LegacyTransaction::try_from(tx);
        let unsigned_tx = unsigned_tx?;
        let account = self.account(symbol, address);
        if account.is_none() {
            return Err(Error::CannotFoundAccount.into());
        }

        let private_key = self
            .find_private_key(&symbol, &address)
            .map_err(|_| Error::CannotGetPrivateKey)?;

        let ecdsa = unsigned_tx
            .ecdsa(private_key.to_bytes().as_slice())
            .map_err(|_| Error::PrivateKeyUnConvert)?;

        let signature = hex::encode(unsigned_tx.sign(&ecdsa));

        Ok(EthereumTxOut { signature })
    }
}

#[test]
fn test_sign() {
    let input = EthereumTxIn {
        nonce: "0".to_string(),
        to: "132D1eA7EF895b6834D25911656f434d7167093C".to_string(),
        value: U256::zero().to_string(),
        gas_price: "1000".to_string(),
        gas: "21240".to_string(),
        data: "7f7465737432000000000000000000000000000000000000000000000000000000600057"
            .to_string(),
        network: "ROPSTEN".to_string(),
    };

    let raw_tx = LegacyTransaction::try_from(&input).unwrap();
    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(
        &hex::decode("2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4").unwrap(),
    );
    let private_key = H256::from_slice(&data);
    let chain_id = chain_id_from_network(input.network.as_str()).unwrap();
    let raw_rlp_bytes = raw_tx.sign(&private_key, &chain_id);
    let result = "f886808210008302124094132d1ea7ef895b6834d25911656f434d7167093c80a47f746573743200000000000000000000000000000000000000000000000000000060005729a00bba7863888f7a29098458d405f95c95ce30d9b36d259af54d064776a10a283ba0076cddae3a17c3dae4ab09454331b3b6218085d1542e4afeadbc0e8986b4d62e";
    assert_eq!(result, hex::encode(raw_rlp_bytes));
}
