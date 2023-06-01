use parking_lot::RwLock;

pub use anyhow::Result;

/// Ethereum chain info
#[derive(Clone)]
pub struct ChainInfo {
    pub network: String,
    pub network_id: i32,
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
            network: "ROPSTEN".to_string(),
            network_id: 3,
            chain_id: 3,
        });
        chain_infos.push(ChainInfo {
            network: "RINKEBY".to_string(),
            network_id: 4,
            chain_id: 4,
        });
        chain_infos.push(ChainInfo {
            network: "GOERLI".to_string(),
            network_id: 5,
            chain_id: 5,
        });
        chain_infos.push(ChainInfo {
            network: "KOVAN".to_string(),
            network_id: 42,
            chain_id: 42,
        });
        chain_infos.push(ChainInfo {
            network: "BNBMAIN".to_string(),
            network_id: 56,
            chain_id: 56,
        });
        chain_infos.push(ChainInfo {
            network: "BNBTEST".to_string(),
            network_id: 97,
            chain_id: 97,
        });
        chain_infos.push(ChainInfo {
            network: "POLYGON".to_string(),
            network_id: 137,
            chain_id: 137,
        });
        chain_infos.push(ChainInfo {
            network: "MUMBAI".to_string(),
            network_id: 80001,
            chain_id: 80001,
        });
        chain_infos.push(ChainInfo {
            network: "ARBITRUM".to_string(),
            network_id: 42161,
            chain_id: 42161,
        });
        chain_infos.push(ChainInfo {
            network: "ARB_GEO".to_string(),
            network_id: 421613,
            chain_id: 421613,
        });
        chain_infos.push(ChainInfo {
            network: "OPTIMISM".to_string(),
            network_id: 10,
            chain_id: 10,
        });
        chain_infos.push(ChainInfo {
            network: "AVALANCHE".to_string(),
            network_id: 43114,
            chain_id: 43114,
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
            network: "HARMONY".to_string(),
            network_id: 1666600000,
            chain_id: 1666600000,
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
