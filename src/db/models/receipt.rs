use ethers::types::TransactionReceipt;
use field_count::FieldCount;

use crate::utils::format::{format_address, format_hash};

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Reverted,
    Succeed,
    Pending,
}

#[derive(Debug, Clone, FieldCount)]
pub struct DatabaseReceipt {
    pub contract_address: Option<String>,
    pub cumulative_gas_used: i64,
    pub effective_gas_price: Option<i64>,
    pub gas_used: i64,
    pub hash: String,
    pub status: TransactionStatus,
}

impl DatabaseReceipt {
    pub fn from_rpc(receipt: &TransactionReceipt) -> Self {
        let contract_address: Option<String> = match receipt.contract_address {
            None => None,
            Some(contract_address) => Some(format_address(contract_address)),
        };

        let status: TransactionStatus = match receipt.status {
            None => TransactionStatus::Succeed,
            Some(status) => {
                let status_number = status.as_u64() as i64;

                if status_number == 0 {
                    TransactionStatus::Reverted
                } else {
                    TransactionStatus::Succeed
                }
            }
        };

        let effective_gas_price: Option<i64> = match receipt.effective_gas_price {
            None => None,
            Some(effective_gas_price) => Some(effective_gas_price.as_u64() as i64),
        };

        let gas_used: i64 = match receipt.gas_used {
            None => 0,
            Some(gas_used) => gas_used.as_u64() as i64,
        };

        Self {
            contract_address,
            cumulative_gas_used: receipt.cumulative_gas_used.as_u64() as i64,
            effective_gas_price,
            gas_used,
            hash: format_hash(receipt.transaction_hash),
            status,
        }
    }
}