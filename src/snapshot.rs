use crate::ocex::AccountInfo;
use crate::withdrawal::Withdrawal;
use frame_support::BoundedVec;
use sp_core::H256;
use sp_runtime::traits::Zero;
use std::collections::BTreeMap;
use frame_support::storage::bounded_btree_map::BoundedBTreeMap;


use crate::AssetId;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::Get;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Provides maximum number of accounts possible in enclave data dump
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
pub struct Fees<Balance: Zero + Clone> {
    pub asset: AssetId,
    pub amount: Balance,
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(SnapshotAccLimit, WithdrawalLimit, AssetsLimit))]
pub struct EnclaveSnapshot<
    Account: std::cmp::Ord,
    Balance: Zero + Clone,
    WithdrawalLimit: Get<u32>,
    AssetsLimit: Get<u32>,
> {
    /// Serial number of snapshot.
    pub snapshot_number: u32,
    /// Hash of the balance snapshot dump made by enclave. ( dump contains all the accounts in enclave )
    pub merkle_root: H256,
    /// Withdrawals
    pub withdrawals: BTreeMap<Account, BoundedVec<Withdrawal<Account, Balance>, WithdrawalLimit>>,
    /// Fees collected by the operator
    pub fees: BoundedVec<Fees<Balance>, AssetsLimit>,
}

impl<Account: std::cmp::Ord, Balance: Zero + Clone, WithdrawalLimit: Get<u32>, AssetsLimit: Get<u32>> PartialEq
    for EnclaveSnapshot<Account, Balance, WithdrawalLimit, AssetsLimit>
{
    fn eq(&self, other: &Self) -> bool {
        self.snapshot_number == other.snapshot_number
    }
}

mod test{
    use sp_runtime::AccountId32;
    use std::convert::TryInto;
    use std::convert::From;
    use std::collections::BTreeMap;
    use crate::{AccountId, snapshot::AccountInfo, Balance, ProxyLimit, snapshot::{H256, EnclaveSnapshot}, BoundedVec, AssetsLimit, WithdrawalLimit, withdrawal::Withdrawal, assets::AssetId};
    use frame_support::bounded_vec;

    #[test]
    fn snapshot_serialize_deserialize(){
        let account1: AccountId32 = [0_u8;32].into();
        let account2: AccountId32 = [2_u8;32].into();
        let mut withdrawals:  BTreeMap<AccountId, BoundedVec<Withdrawal<AccountId, Balance>, WithdrawalLimit>> = BTreeMap::new();
        let withdrawal: Withdrawal<AccountId, Balance> = Withdrawal{
            main_account: account1.clone(),
            amount: 100_u128,
            asset: AssetId::polkadex
        };
        withdrawals.insert(account1, bounded_vec![withdrawal.clone(), withdrawal.clone()]);
        withdrawals.insert(account2, bounded_vec![withdrawal.clone()]);


        let snapshot = EnclaveSnapshot::<AccountId, Balance, WithdrawalLimit, AssetsLimit>{
            snapshot_number: 0,
            merkle_root: H256([0_u8;32]),
            withdrawals,
            fees: bounded_vec![]
        };
        let serialized = serde_json::to_string(&snapshot).unwrap();
        let deserialized: EnclaveSnapshot<AccountId, Balance, WithdrawalLimit, AssetsLimit> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, snapshot);
    }
}
