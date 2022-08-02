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
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use std::fmt::{Display, Formatter};
use serde::{de, Deserializer, Serializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

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
#[cfg_attr(feature = "std", derive(Deserialize))]
#[serde(tag = "asset_id")]
pub enum AssetId {
    /// Generic enumerated assed
    /// Range 0 - 0x00000000FFFFFFFF (2^32)-1 is reserved for protected tokens
    /// the values under 1000 are used for ISO 4217 Numeric Curency codes
    asset(u128),
    /// PDEX the native currency of the chain
    polkadex,
}

impl Serialize for AssetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            AssetId::asset(i) => serializer.serialize_u128(i),
            AssetId::polkadex => serializer.serialize_unit_variant("polkadex", 1, "polkadex"),
        }
    }
}

/*impl<'de> Deserialize<'de> for AssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct AssetIdVisitor;

        impl<'de> Visitor<'de> for AssetVisitor {

            type Value = AssetId;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 2^127")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, serde::de::Error>
                where
            A: EnumAccess<'de>
            {
                match data.variant() {
                    AssetId::asset(n) =>
                }
            }
        }

        const VARIANTS: &'static [&'static str] = &["asset", "polkadex"];
        deserializer.deserialize_enum("AssetId", VARIANTS, AssetVisitor)
    }
}
*/
impl Display for AssetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetId::polkadex => write!(f, "PDEX"),
            AssetId::asset(id) => write!(f, "{:?}", id),
        }
    }
}
