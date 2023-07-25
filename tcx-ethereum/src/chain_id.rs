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
            network: "mainnet".to_string(),
            network_id: 1,
            chain_id: 1,
        });
        chain_infos.push(ChainInfo {
            network: "sepolia".to_string(),
            network_id: 58008,
            chain_id: 58008,
        });
        chain_infos.push(ChainInfo {
            network: "goerli".to_string(),
            network_id: 5,
            chain_id: 5,
        });
        chain_infos.push(ChainInfo {
            network: "kovan".to_string(),
            network_id: 42,
            chain_id: 42,
        });
        chain_infos.push(ChainInfo {
            network: "bnbsmartchain-mainnet".to_string(),
            network_id: 56,
            chain_id: 56,
        });
        chain_infos.push(ChainInfo {
            network: "bnbsmartchain-testnet".to_string(),
            network_id: 97,
            chain_id: 97,
        });
        chain_infos.push(ChainInfo {
            network: "polygon-mainnet".to_string(),
            network_id: 137,
            chain_id: 137,
        });
        chain_infos.push(ChainInfo {
            network: "polygon-mumbai".to_string(),
            network_id: 80001,
            chain_id: 80001,
        });
        chain_infos.push(ChainInfo {
            network: "arbitrum-mainnet".to_string(),
            network_id: 42161,
            chain_id: 42161,
        });
        chain_infos.push(ChainInfo {
            network: "arbitrum-goerli".to_string(),
            network_id: 421613,
            chain_id: 421613,
        });
        chain_infos.push(ChainInfo {
            network: "optimism-mainnet".to_string(),
            network_id: 10,
            chain_id: 10,
        });
        chain_infos.push(ChainInfo {
            network: "optimism-goerli".to_string(),
            network_id: 420,
            chain_id: 420,
        });
        chain_infos.push(ChainInfo {
            network: "avalanche-mainnet".to_string(),
            network_id: 43114,
            chain_id: 43114,
        });
        chain_infos.push(ChainInfo {
            network: "avalanche-fuji".to_string(),
            network_id: 43113,
            chain_id: 43113,
        });
        chain_infos.push(ChainInfo {
            network: "moonbeam".to_string(),
            network_id: 1284,
            chain_id: 1284,
        });
        chain_infos.push(ChainInfo {
            network: "moonriver".to_string(),
            network_id: 1285,
            chain_id: 1285,
        });
        chain_infos.push(ChainInfo {
            network: "fantom".to_string(),
            network_id: 250,
            chain_id: 250,
        });
        chain_infos.push(ChainInfo {
            network: "base-goerli".to_string(),
            network_id: 8453,
            chain_id: 8453,
        });
        chain_infos.push(ChainInfo {
            network: "harmony".to_string(),
            network_id: 1666600000,
            chain_id: 1666600000,
        });
        chain_infos.push(ChainInfo {
            network: "celo-mainnet".to_string(),
            network_id: 42220,
            chain_id: 42220,
        });
        chain_infos.push(ChainInfo {
            network: "celo-alfajores".to_string(),
            network_id: 44787,
            chain_id: 44787,
        });
        chain_infos.push(ChainInfo {
            network: "linea-goerli".to_string(),
            network_id: 59140,
            chain_id: 59140,
        });
        chain_infos.push(ChainInfo {
            network: "palm-mainnet".to_string(),
            network_id: 11297108109,
            chain_id: 11297108109,
        });
        chain_infos.push(ChainInfo {
            network: "palm-testnet".to_string(),
            network_id: 11297108099,
            chain_id: 11297108099,
        });

        chain_infos.push(ChainInfo {
            network: "aurora-mainnet".to_string(),
            network_id: 1313161554,
            chain_id: 1313161554,
        });
        chain_infos.push(ChainInfo {
            network: "aurora-testnet".to_string(),
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
