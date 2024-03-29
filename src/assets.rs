// This file is part of Polkadex.

// Copyright (C) 2020-2021 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{de, Deserializer, Serializer};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use std::fmt::{Display, Formatter};

/// Enumerated asset on chain
#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
pub enum AssetId {
    /// Generic enumerated assed
    /// Range 0 - 0x00000000FFFFFFFF (2^32)-1 is reserved for protected tokens
    /// the values under 1000 are used for ISO 4217 Numeric Curency codes
    asset(u128),
    /// PDEX the native currency of the chain
    polkadex,
}

impl Display for AssetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetId::polkadex => write!(f, "PDEX"),
            AssetId::asset(id) => write!(f, "{:?}", id),
        }
    }
}

#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[serde(tag = "asset_id")]
pub enum HashAssetId {
    /// Generic enumerated assed
    /// Range 0 - 0x00000000FFFFFFFF (2^32)-1 is reserved for protected tokens
    /// the values under 1000 are used for ISO 4217 Numeric Curency codes
    asset(u128),
    /// PDEX the native currency of the chain
    polkadex,
}

impl Serialize for HashAssetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            HashAssetId::asset(i) => serializer.serialize_u128(i),
            HashAssetId::polkadex => serializer.serialize_unit_variant("asset_id", 1, "polkadex"),
        }
    }
}

impl Display for HashAssetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAssetId::polkadex => write!(f, "PDEX"),
            HashAssetId::asset(id) => write!(f, "{:?}", id),
        }
    }
}

impl Into<AssetId> for HashAssetId {
    fn into(self) -> AssetId {
        match self {
            HashAssetId::polkadex => AssetId::polkadex,
            HashAssetId::asset(n) => AssetId::asset(n)
        }
    }
}

impl Into<HashAssetId> for AssetId {
    fn into(self) -> HashAssetId {
        match self {
            AssetId::polkadex => HashAssetId::polkadex,
            AssetId::asset(n) => HashAssetId::asset(n)
        }
    }
}

