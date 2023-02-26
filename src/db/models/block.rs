use ethers::{
    types::{Block, Transaction, H160},
    utils::format_units,
};
use field_count::FieldCount;

#[derive(Debug, Clone)]
pub enum BlockStatus {
    Unfinalized,
    Secure,
    Finalized,
}

use crate::utils::format::{format_address, format_hash, format_nonce, format_number};

#[derive(Debug, Clone, FieldCount)]
pub struct DatabaseBlock {
    pub base_fee_per_gas: Option<f64>,
    pub chain: i64,
    pub difficulty: String,
    pub extra_data: Vec<u8>,
    pub gas_limit: i64,
    pub gas_used: i64,
    pub hash: String,
    pub logs_bloom: Vec<u8>,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: i64,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: i32,
    pub state_root: String,
    pub status: BlockStatus,
    pub timestamp: i64,
    pub total_difficulty: String,
    pub transactions: i32,
    pub transactions_root: String,
    pub uncles: Vec<String>,
}

impl DatabaseBlock {
    pub fn from_rpc(block: &Block<Transaction>, chain: i64) -> Self {
        let base_fee_per_gas: Option<f64> = match block.base_fee_per_gas {
            None => None,
            Some(base_fee_per_gas) => Some(
                format_units(base_fee_per_gas, 18)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            ),
        };

        let nonce: String = match block.nonce {
            None => String::from("0"),
            Some(nonce) => format_nonce(nonce),
        };

        let uncles = block
            .uncles
            .clone()
            .into_iter()
            .map(|uncle| format_hash(uncle))
            .collect();

        let mix_hash: String = match block.mix_hash {
            None => String::from("0x"),
            Some(mix_hash) => format_hash(mix_hash),
        };

        let hash: String = match block.hash {
            None => String::from("0x"),
            Some(hash) => format_hash(hash),
        };

        let number: i64 = match block.number {
            None => 0,
            Some(number) => number.as_u64() as i64,
        };

        let size: i32 = match block.size {
            None => 0,
            Some(size) => size.as_u32() as i32,
        };

        let total_difficulty: String = match block.total_difficulty {
            None => String::from("0x"),
            Some(total_difficulty) => format_number(total_difficulty),
        };

        let miner: String = match block.author {
            None => format_address(H160::zero()),
            Some(author) => format_address(author),
        };

        Self {
            base_fee_per_gas,
            chain,
            difficulty: format_number(block.difficulty),
            extra_data: block.extra_data.to_vec(),
            gas_limit: block.gas_limit.as_u64() as i64,
            gas_used: block.gas_used.as_u64() as i64,
            hash,
            logs_bloom: block.logs_bloom.unwrap().as_bytes().to_vec(),
            miner,
            mix_hash,
            nonce,
            number,
            parent_hash: format_hash(block.parent_hash),
            receipts_root: format_hash(block.receipts_root),
            sha3_uncles: format_hash(block.uncles_hash),
            size,
            status: BlockStatus::Unfinalized,
            state_root: format_hash(block.state_root),
            timestamp: block.timestamp.as_u64() as i64,
            transactions_root: format_hash(block.transactions_root),
            total_difficulty,
            transactions: block.transactions.len() as i32,
            uncles,
        }
    }
}