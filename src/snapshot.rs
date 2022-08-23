use crate::ocex::AccountInfo;
use crate::withdrawal::Withdrawal;
use frame_support::BoundedVec;
use sp_core::H256;
use sp_runtime::traits::Zero;
use sp_std::collections::btree_map::BTreeMap;
use codec::{Decode, Encode,MaxEncodedLen};
use frame_support::storage::bounded_btree_map::BoundedBTreeMap;
use frame_support::traits::Get;
use scale_info::TypeInfo;
use crate::{AssetId};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Provides maximum number of accounts possible in enclave data dump
pub struct AccountInfoDumpLimit;
impl Get<u32> for AccountInfoDumpLimit {
    fn get() -> u32 {
        10000000
    }
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EnclaveAccountInfoDump<AccountId: Ord, Balance: Zero + Clone, ProxyLimit: Get<u32>> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// All Accounts present in enclave
    pub accounts: BTreeMap<AccountId, AccountInfo<AccountId, Balance, ProxyLimit>>,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Fees<Balance: Zero + Clone>{
    pub asset: AssetId,
    pub amount: Balance
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std",derive(Debug))]
#[scale_info(skip_type_params(SnapshotAccLimit, WithdrawalLimit,AssetsLimit ))]
pub struct EnclaveSnapshot<Account: Ord, Balance: Zero + Clone, WithdrawalLimit: Get<u32>, AssetsLimit: Get<u32>, SnapshotAccLimit: Get<u32>> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// Hash of the balance snapshot dump made by enclave. ( dump contains all the accounts in enclave )
    pub merkle_root: H256,
    /// Withdrawals
    pub withdrawals: BoundedBTreeMap<Account, BoundedVec<Withdrawal<Account, Balance>, WithdrawalLimit>, SnapshotAccLimit>,
    /// Fees collected by the operator
    pub fees: BoundedVec<Fees<Balance>,AssetsLimit>
}

#[cfg(feature = "std")]
impl<Account: Ord,
    Balance: Zero + Clone,
    WithdrawalLimit: Get<u32>,
    AssetsLimit: Get<u32>,
    SnapshotAccLimit: Get<u32>
> TryFrom<EnclaveSnapshotStd<Account, Balance, WithdrawalLimit, AssetsLimit>> for
EnclaveSnapshot<Account, Balance, WithdrawalLimit, AssetsLimit, SnapshotAccLimit> {
    type Error = ();

    fn try_from(value: EnclaveSnapshotStd<Account, Balance, WithdrawalLimit, AssetsLimit>) -> Result<Self, Self::Error> {
        Ok(EnclaveSnapshot {
            snapshot_number: value.snapshot_number,
            merkle_root: value.merkle_root,
            withdrawals: BoundedBTreeMap::try_from(value.withdrawals)?,
            fees: value.fees
        })
    }
}




/// This is for use passing data from enclave to relayer via RPC, it's short cut.
#[cfg_attr(feature = "std",derive(Deserialize,Serialize, Debug))]
#[cfg(feature = "std")]
pub struct EnclaveSnapshotStd<Account: Ord, Balance: Zero + Clone, WithdrawalLimit: Get<u32>, AssetsLimit: Get<u32>> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// Hash of the balance snapshot dump made by enclave. ( dump contains all the accounts in enclave )
    pub merkle_root: H256,
    /// Withdrawals
    pub withdrawals: BTreeMap<Account, BoundedVec<Withdrawal<Account, Balance>, WithdrawalLimit>>,
    /// Fees collected by the operator
    pub fees: BoundedVec<Fees<Balance>,AssetsLimit>
}

impl<Account: Ord, Balance: Zero + Clone, WithdrawalLimit: Get<u32>, AssetsLimit: Get<u32>, SnapshotAccLimit: Get<u32>> PartialEq
    for EnclaveSnapshot<Account, Balance, WithdrawalLimit,AssetsLimit, SnapshotAccLimit>
{
    fn eq(&self, other: &Self) -> bool {
        self.snapshot_number == other.snapshot_number
    }
}
