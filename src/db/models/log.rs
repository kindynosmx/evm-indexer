use ethers::types::Log;
use field_count::FieldCount;

use crate::utils::format::{format_address, format_hash};

#[derive(Debug, Clone, FieldCount)]
pub struct DatabaseLog {
    pub address: String,
    pub chain: i64,
    pub data: Vec<u8>,
    pub hash: String,
    pub log_index: i32,
    pub removed: bool,
    pub topics: Vec<String>,
    pub transaction_log_index: i32,
}

impl DatabaseLog {
    pub fn from_rpc(log: Log, chain: i64) -> Self {
        Self {
            address: format_address(log.address),
            chain,
            topics: log
                .topics
                .clone()
                .into_iter()
                .map(|topic| format_hash(topic))
                .collect(),
            data: log.data.to_vec(),
            hash: format_hash(log.transaction_hash.unwrap()),
            removed: log.removed.unwrap(),
            log_index: log.log_index.unwrap().as_u32() as i32,
            transaction_log_index: log.transaction_log_index.unwrap().as_u32() as i32,
        }
    }
}
