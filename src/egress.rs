use crate::snapshot::{EnclaveAccountInfoDump, EnclaveSnapshot};
use crate::Signature;
use frame_support::traits::Get;
use frame_support::BoundedVec;
use sp_runtime::traits::Zero;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EgressMessages {
    RegisterEnclave(BoundedVec<u8, UnpaddedReportSize>),
}

/// Provides size of the unpadded report
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UnpaddedReportSize;
impl Get<u32> for UnpaddedReportSize {
    fn get() -> u32 {
        432
    }
}
