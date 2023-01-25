use crate::ocex::{OCEXConfig, TradingPairConfig};
use crate::AssetId;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::Get;
use frame_support::BoundedVec;
use rust_decimal::Decimal;
use scale_info::TypeInfo;
use sp_core::H256;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IngressMessages<AccountId> {
    // Start Enclave
    StartEnclave(OCEXConfig<AccountId>),
    // Open Trading Pair
    OpenTradingPair(TradingPairConfig),
    // Update Trading Pair Config
    UpdateTradingPair(TradingPairConfig),
    // Register User ( main, proxy)
    RegisterUser(AccountId, AccountId),
    // Main Acc, Assetid, Amount
    Deposit(AccountId, AssetId, Decimal),
    // Main Acc, Proxy Account
    AddProxy(AccountId, AccountId),
    // Main Acc, Proxy Account
    RemoveProxy(AccountId, AccountId),
    // Enclave registration confirmation
    EnclaveRegistered(AccountId),
    // Shutdown Exchange
    Shutdown,
    // Close Trading Pair
    CloseTradingPair(TradingPairConfig),
    // Latest snapshot (MerkelRoot, snapshot_no)
    LastestSnapshot(H256, u32),
    // Resetting the balances of Account
    SetFreeReserveBalanceForAccounts(BoundedVec<HandleBalance<AccountId>, HandleBalanceLimit>),
    // Changing the exchange state in order-book
    SetExchangeState(bool),
    // Withdrawal from Chain to OrderBook
    DirectWithdrawal(AccountId, AssetId, Decimal)
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HandleBalance<AccountId> {
    pub main_account: AccountId,
    pub asset_id: AssetId,
    pub free: u128,
    pub reserve: u128,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HandleBalanceLimit;

impl Get<u32> for HandleBalanceLimit {
    //ToDo: Set an arbitrary value to 1000.
    fn get() -> u32 {
        1000
    }
}
