use crate::assets::AssetId;
use codec::{Decode, Encode, MaxEncodedLen};
use rust_decimal::Decimal;
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
use crate::{AccountId, BlockNumber, Header};

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Withdrawal<AccountId> {
    pub main_account: AccountId,
    pub amount: Decimal,
    pub asset: AssetId,
    pub event_id: u64,
    pub fees: Decimal,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct WithdrawalPayload {
    pub asset_id: AssetId,
    pub amount: Decimal,
    pub user: AccountId,
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone, TypeInfo)]
pub struct Withdrawals {
    pub withdrawals: Vec<WithdrawalPayload>,
    pub nonce: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug, Encode, Decode, TypeInfo)]
pub struct SnapshotSummary {
    // Last synced blocknumber
    pub last_block: BlockNumber,
    // Snapshot Number
    pub snapshot_number: u64,
    // Hash of the enclave state
    pub enclave_state_hash: H256,
    // The header that was used to initialise the enclave
    pub initialization_header: Header,
    // Pending Withdrawals
    pub withdrawals_processed: Withdrawals,
}
