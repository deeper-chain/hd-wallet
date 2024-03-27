use parking_lot::RwLock;

pub use anyhow::Result;

/// Ethereum chain info
#[derive(Clone)]
pub struct ChainInfo {
    pub network: String,
    pub network_id: u64,
    pub chain_id: u64,
}

lazy_static! {
    static ref CHAIN_INFOS: RwLock<Vec<ChainInfo>> = {
        let mut chain_infos = Vec::new();
        chain_infos.push(ChainInfo {
            network: "MAINNET".to_string(),
            network_id: 1,
            chain_id: 1,
        });
        chain_infos.push(ChainInfo {
            network: "SEPOLIA".to_string(),
            network_id: 11155111,
            chain_id: 11155111,
        });
        chain_infos.push(ChainInfo {
            network: "HOLESKY".to_string(),
            network_id: 17000,
            chain_id: 17000,
        });
        chain_infos.push(ChainInfo {
            network: "GOERLI".to_string(),
            network_id: 5,
            chain_id: 5,
        });
        chain_infos.push(ChainInfo {
            network: "KOVAN".to_string(),
            network_id: 2222,
            chain_id: 2222,
        });
        chain_infos.push(ChainInfo {
            network: "BNBSMARTCHAIN-MAINNET".to_string(),
            network_id: 56,
            chain_id: 56,
        });
        chain_infos.push(ChainInfo {
            network: "BNBSMARTCHAIN-TESTNET".to_string(),
            network_id: 97,
            chain_id: 97,
        });
        chain_infos.push(ChainInfo {
            network: "POLYGON-MAINNET".to_string(),
            network_id: 137,
            chain_id: 137,
        });
        chain_infos.push(ChainInfo {
            network: "POLYGON-MUMBAI".to_string(),
            network_id: 80001,
            chain_id: 80001,
        });
        chain_infos.push(ChainInfo {
            network: "ARBITRUM-MAINNET".to_string(),
            network_id: 42161,
            chain_id: 42161,
        });
        chain_infos.push(ChainInfo {
            network: "ARBITRUM-GOERLI".to_string(),
            network_id: 421613,
            chain_id: 421613,
        });
        chain_infos.push(ChainInfo {
            network: "OPTIMISM-MAINNET".to_string(),
            network_id: 10,
            chain_id: 10,
        });
        chain_infos.push(ChainInfo {
            network: "OPTIMISM-GOERLI".to_string(),
            network_id: 420,
            chain_id: 420,
        });
        chain_infos.push(ChainInfo {
            network: "AVALANCHE-MAINNET".to_string(),
            network_id: 43114,
            chain_id: 43114,
        });
        chain_infos.push(ChainInfo {
            network: "AVALANCHE-FUJI".to_string(),
            network_id: 43113,
            chain_id: 43113,
        });
        chain_infos.push(ChainInfo {
            network: "MOONBEAM".to_string(),
            network_id: 1284,
            chain_id: 1284,
        });
        chain_infos.push(ChainInfo {
            network: "MOONRIVER".to_string(),
            network_id: 1285,
            chain_id: 1285,
        });
        chain_infos.push(ChainInfo {
            network: "FANTOM".to_string(),
            network_id: 250,
            chain_id: 250,
        });
        chain_infos.push(ChainInfo {
            network: "BASE".to_string(),
            network_id: 8453,
            chain_id: 8453,
        });
        chain_infos.push(ChainInfo {
            network: "HARMONY".to_string(),
            network_id: 1666600000,
            chain_id: 1666600000,
        });
        chain_infos.push(ChainInfo {
            network: "CELO-MAINNET".to_string(),
            network_id: 42220,
            chain_id: 42220,
        });
        chain_infos.push(ChainInfo {
            network: "CELO-ALFAJORES".to_string(),
            network_id: 44787,
            chain_id: 44787,
        });
        chain_infos.push(ChainInfo {
            network: "LINEA-GOERLI".to_string(),
            network_id: 59140,
            chain_id: 59140,
        });
        chain_infos.push(ChainInfo {
            network: "PALM-MAINNET".to_string(),
            network_id: 11297108109,
            chain_id: 11297108109,
        });
        chain_infos.push(ChainInfo {
            network: "PALM-TESTNET".to_string(),
            network_id: 11297108099,
            chain_id: 11297108099,
        });

        chain_infos.push(ChainInfo {
            network: "AURORA-MAINNET".to_string(),
            network_id: 1313161554,
            chain_id: 1313161554,
        });
        chain_infos.push(ChainInfo {
            network: "AURORA-TESTNET".to_string(),
            network_id: 1313161555,
            chain_id: 1313161555,
        });

        RwLock::new(chain_infos)
    };
}

pub fn chain_id_from_network(network: &str) -> Result<u64> {
    let chain_infos = CHAIN_INFOS.read();
    let mut res: Vec<u64> = chain_infos
        .iter()
        .filter(|x| x.network.as_str() == network)
        .map(|x| x.chain_id)
        .collect::<Vec<u64>>();
    if res.len() > 0 {
        Ok(res.pop().unwrap())
    } else {
        Err(anyhow::anyhow!("No chain id for network"))
    }
}
