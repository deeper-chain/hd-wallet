use crate::curve::CurveType;
use crate::Result;

use parking_lot::RwLock;

/// Blockchain basic config
///
/// NOTE: Unique key field is `symbol`
#[derive(Clone, Debug)]
pub struct CoinInfo {
    pub coin: String,
    pub derivation_path: String,
    pub curve: CurveType,
    pub network: String,
    pub seg_wit: String,
}

lazy_static! {
    static ref COIN_INFOS: RwLock<Vec<CoinInfo>> = {
        let mut coin_infos = Vec::new();
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/84'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/84'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/49'/0'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "P2SHWPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOIN".to_string(),
            derivation_path: "m/49'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "P2SHWPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/145'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/49'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/49'/1'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "P2WPKH".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "TRON".to_string(),
            derivation_path: "m/44'/195'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "NERVOS".to_string(),
            derivation_path: "m/44'/309'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "POLKADOT".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "POLKADOT".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::ED25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "KUSAMA".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "KUSAMA".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::ED25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "DEEPER".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::SubSr25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "DEEPER".to_string(),
            derivation_path: "".to_string(),
            curve: CurveType::ED25519,
            network: "".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "TEZOS".to_string(),
            derivation_path: "m/44'/1729'/0'/0'".to_string(),
            curve: CurveType::ED25519,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/44'/461'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/44'/461'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/2334/461/0/0".to_string(),
            curve: CurveType::BLS,
            network: "MAINNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "FILECOIN".to_string(),
            derivation_path: "m/2334/461/0/0".to_string(),
            curve: CurveType::BLS,
            network: "TESTNET".to_string(),
            seg_wit: "".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "GOERLI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "SEPOLIA".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "HOLESKY".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "KOVAN".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/714'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "BNBSMARTCHAIN-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/714'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "BNBSMARTCHAIN-TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });

        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "POLYGON-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "POLYGON-MUMBAI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "ARBITRUM-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "ARBITRUM-GOERLI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "OPTIMISM-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "OPTIMISM-GOERLI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "AVALANCHE-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "AVALANCHE-FUJI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MOONBEAM".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MOONRIVER".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "FANTOM".to_string(),
            seg_wit: "NONE".to_string(),
        });

        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "HARMONY".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "CELO-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "CELO-ALFAJORES".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "LINEA-GOERLI".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "PALM-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "PALM-TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "AURORA-MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "AURORA-TESTNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "ETHEREUM".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "BASE".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "SUI".to_string(),
            derivation_path: "m/44'/784'/0'/0'/0'".to_string(),
            curve: CurveType::ED25519,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "SUI".to_string(),
            derivation_path: "m/54'/784'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        coin_infos.push(CoinInfo {
            coin: "SOLANA".to_string(),
            derivation_path: "m/44'/501'/0'/0'/0'".to_string(),
            curve: CurveType::ED25519,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        });
        RwLock::new(coin_infos)
    };
}

pub fn coin_info_from_param(
    chain_type: &str,
    network: &str,
    seg_wit: &str,
    curve: &str,
) -> Result<CoinInfo> {
    let coin_infos = COIN_INFOS.read();
    let mut coins = coin_infos
        .iter()
        .filter(|x| {
            x.coin.as_str() == chain_type
                && (x.network.as_str() == network || network.is_empty())
                && (x.seg_wit.as_str() == seg_wit || seg_wit.is_empty())
                && (x.curve.as_str() == curve || curve.is_empty())
        })
        .map(|x| x.clone())
        .collect::<Vec<CoinInfo>>();

    if coins.is_empty() {
        Err(anyhow::anyhow!("unsupported_chain"))
    } else {
        Ok(coins.pop().expect("coin_info_from_param"))
    }
}
