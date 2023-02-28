mod address;
mod hash;
mod serializer;
mod signer;
mod transaction;
mod transaction_helper;

pub use address::CkbAddress;
pub use serializer::Serializer;
use tcx_chain::Result;
use thiserror::Error;
pub use transaction::{CachedCell, CellInput, CkbTxInput, CkbTxOutput, OutPoint, Script, Witness};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid_output_point")]
    InvalidOutputPoint,

    #[error("invalid_outputs_data_length")]
    InvalidOutputsDataLength,

    #[error("required_witness")]
    RequiredWitness,

    #[error("invalid_input_cells")]
    InvalidInputCells,

    #[error("required_output_data")]
    RequiredOutputsData,

    #[error("witness_group_empty")]
    WitnessGroupEmpty,

    #[error("witness_empty")]
    WitnessEmpty,

    #[error("invalid_tx_hash")]
    InvalidTxHash,

    #[error("invalid_hash_type")]
    InvalidHashType,

    #[error("cell_input_not_cached")]
    CellInputNotCached,

    #[error("invalid_hex_value")]
    InvalidHexValue,
}

pub fn hex_to_bytes(value: &str) -> Result<Vec<u8>> {
    let result = if value.starts_with("0x") || value.starts_with("0X") {
        hex::decode(&value[2..])
    } else {
        hex::decode(&value[..])
    };

    result.map_err(|_| Error::InvalidHexValue.into())
}

#[cfg(test)]
mod tests {
    use crate::hex_to_bytes;

    #[test]
    pub fn hex_convert() {
        let v: Vec<u8> = vec![];
        assert_eq!(v, hex_to_bytes("0x").unwrap());
        assert_eq!(vec![0x01], hex_to_bytes("0x01").unwrap());
        assert_eq!(vec![0x02], hex_to_bytes("0x02").unwrap());
        assert_eq!(vec![0x02, 0x11], hex_to_bytes("0x0211").unwrap());
    }
}
