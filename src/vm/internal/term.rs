// Copyright (c) 2022 Octavian Oncescu
// Copyright (c) 2022-2023 The Purplecoin Core developers
// Licensed under the Apache License, Version 2.0 see LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license, see
// LICENSE-MIT or http://opensource.org/licenses/MIT

use bincode::{Decode, Encode};
use ibig::ops::Abs;
use ibig::{ibig, ubig, IBig, UBig};
use num_traits::identities::Zero;
use std::fmt;

const WORD_SIZE: usize = 8; // 8 bytes on 64bit machines
const EMPTY_VEC_HEAP_SIZE: usize = 3 * WORD_SIZE; // 3 words

#[derive(Clone, PartialEq, Eq)]
pub enum VmTerm {
    Hash160([u8; 20]),
    Hash256([u8; 32]),
    Hash512([u8; 64]),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),
    UnsignedBig(UBig),
    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),
    Signed128(i128),
    SignedBig(IBig),
    Hash160Array(Vec<[u8; 20]>),
    Hash256Array(Vec<[u8; 32]>),
    Hash512Array(Vec<[u8; 64]>),
    Unsigned8Array(Vec<u8>),
    Unsigned16Array(Vec<u16>),
    Unsigned32Array(Vec<u32>),
    Unsigned64Array(Vec<u64>),
    Unsigned128Array(Vec<u128>),
    UnsignedBigArray(Vec<UBig>),
    Signed8Array(Vec<i8>),
    Signed16Array(Vec<i16>),
    Signed32Array(Vec<i32>),
    Signed64Array(Vec<i64>),
    Signed128Array(Vec<i128>),
    SignedBigArray(Vec<IBig>),
}

macro_rules! impl_hash_debug {
    ($f:expr, $val:expr, $struc:expr) => {
        $f.debug_tuple($struc).field(&hex::encode($val)).finish()
    };
}

macro_rules! impl_normal_debug {
    ($f:expr, $val:expr, $struc:expr) => {
        $f.debug_tuple($struc).field(&$val).finish()
    };
}

macro_rules! impl_hash_array_debug {
    ($f:expr, $val:expr, $struc:expr) => {{
        let encoded: Vec<_> = $val.iter().map(|e| hex::encode(e)).collect();
        $f.debug_tuple($struc).field(&encoded).finish()
    }};
}

impl fmt::Debug for VmTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hash160(val) => impl_hash_debug!(f, val, "Hash160"),
            Self::Hash256(val) => impl_hash_debug!(f, val, "Hash256"),
            Self::Hash512(val) => impl_hash_debug!(f, val, "Hash512"),
            Self::Unsigned8(val) => impl_normal_debug!(f, val, "Unsigned8"),
            Self::Unsigned16(val) => impl_normal_debug!(f, val, "Unsigned16"),
            Self::Unsigned32(val) => impl_normal_debug!(f, val, "Unsigned32"),
            Self::Unsigned64(val) => impl_normal_debug!(f, val, "Unsigned64"),
            Self::Unsigned128(val) => impl_normal_debug!(f, val, "Unsigned128"),
            Self::UnsignedBig(val) => impl_normal_debug!(f, val, "UnsignedBig"),
            Self::Signed8(val) => impl_normal_debug!(f, val, "Signed8"),
            Self::Signed16(val) => impl_normal_debug!(f, val, "Signed16"),
            Self::Signed32(val) => impl_normal_debug!(f, val, "Signed32"),
            Self::Signed64(val) => impl_normal_debug!(f, val, "Signed64"),
            Self::Signed128(val) => impl_normal_debug!(f, val, "Signed128"),
            Self::SignedBig(val) => impl_normal_debug!(f, val, "SignedBig"),
            Self::Unsigned8Array(val) => impl_normal_debug!(f, val, "Unsigned8Array"),
            Self::Unsigned16Array(val) => impl_normal_debug!(f, val, "Unsigned16Array"),
            Self::Unsigned32Array(val) => impl_normal_debug!(f, val, "Unsigned32Array"),
            Self::Unsigned64Array(val) => impl_normal_debug!(f, val, "Unsigned64Array"),
            Self::Unsigned128Array(val) => impl_normal_debug!(f, val, "Unsigned128Array"),
            Self::UnsignedBigArray(val) => impl_normal_debug!(f, val, "UnsignedBigArray"),
            Self::Signed8Array(val) => impl_normal_debug!(f, val, "Signed8Array"),
            Self::Signed16Array(val) => impl_normal_debug!(f, val, "Signed16Array"),
            Self::Signed32Array(val) => impl_normal_debug!(f, val, "Signed32Array"),
            Self::Signed64Array(val) => impl_normal_debug!(f, val, "Signed64Array"),
            Self::Signed128Array(val) => impl_normal_debug!(f, val, "Signed128Array"),
            Self::SignedBigArray(val) => impl_normal_debug!(f, val, "SignedBigArray"),
            Self::Hash160Array(val) => impl_hash_array_debug!(f, val, "Hash160Array"),
            Self::Hash256Array(val) => impl_hash_array_debug!(f, val, "Hash256Array"),
            Self::Hash512Array(val) => impl_hash_array_debug!(f, val, "Hash512Array"),
        }
    }
}

impl VmTerm {
    pub fn to_bytes(&self) -> Vec<u8> {
        crate::codec::encode_to_vec(self).unwrap()
    }

    pub fn add_one(&mut self) -> Option<()> {
        match self {
            Self::Unsigned8(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Unsigned16(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Unsigned32(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Unsigned64(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Unsigned128(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::UnsignedBig(ref mut val) => {
                *val += 1;
            }
            Self::Signed8(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Signed16(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Signed32(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Signed64(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::Signed128(ref mut val) => {
                *val = val.checked_add(1)?;
            }
            Self::SignedBig(ref mut val) => {
                *val += 1;
            }
            _ => {
                return None;
            }
        }

        Some(())
    }

    pub fn sub_one(&mut self) -> Option<()> {
        match self {
            Self::Unsigned8(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Unsigned16(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Unsigned32(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Unsigned64(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Unsigned128(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::UnsignedBig(ref mut val) => {
                *val -= 1;
            }
            Self::Signed8(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Signed16(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Signed32(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Signed64(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::Signed128(ref mut val) => {
                *val = val.checked_sub(1)?;
            }
            Self::SignedBig(ref mut val) => {
                *val -= 1;
            }
            _ => {
                return None;
            }
        }

        Some(())
    }

    /// Returns the virtual heap size of the term in bytes
    pub fn size(&self) -> usize {
        match self {
            Self::Hash160(_) => 20,
            Self::Hash256(_) => 32,
            Self::Hash512(_) => 64,
            Self::Unsigned8(_) | Self::Signed8(_) => 1,
            Self::Unsigned16(_) | Self::Signed16(_) => 2,
            Self::Unsigned32(_) | Self::Signed32(_) => 4,
            Self::Unsigned64(_) | Self::Signed64(_) => 8,
            Self::Unsigned128(_) | Self::Signed128(_) => 16,
            Self::UnsignedBig(v) => {
                (v.bit_len() >> 3) + EMPTY_VEC_HEAP_SIZE // additional vec allocated by ubig
            }
            Self::SignedBig(v) => {
                let v = v.abs();
                let v: UBig = v.try_into().unwrap();
                (v.bit_len() >> 3) + EMPTY_VEC_HEAP_SIZE + WORD_SIZE // additional vec allocated by ibig plus sign
            }
            Self::Hash160Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 20,
            Self::Hash256Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 32,
            Self::Hash512Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 64,
            Self::Unsigned8Array(val) => EMPTY_VEC_HEAP_SIZE + val.len(),
            Self::Signed8Array(val) => EMPTY_VEC_HEAP_SIZE + val.len(),
            Self::Unsigned16Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 2,
            Self::Signed16Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 2,
            Self::Unsigned32Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 4,
            Self::Signed32Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 4,
            Self::Unsigned64Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 8,
            Self::Signed64Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 8,
            Self::Unsigned128Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 16,
            Self::Signed128Array(val) => EMPTY_VEC_HEAP_SIZE + val.len() * 16,
            Self::UnsignedBigArray(val) => {
                let mut size = EMPTY_VEC_HEAP_SIZE;
                for v in val.iter() {
                    size += (v.bit_len() >> 3) + EMPTY_VEC_HEAP_SIZE; // additional vec allocated by ubig
                }
                size
            }
            Self::SignedBigArray(val) => {
                let mut size = EMPTY_VEC_HEAP_SIZE;
                for v in val.iter() {
                    let v = v.abs();
                    let v: UBig = v.try_into().unwrap();
                    size += (v.bit_len() >> 3) + EMPTY_VEC_HEAP_SIZE + WORD_SIZE;
                    // additional vec allocated by ibig plus sign
                }
                size
            }
        }
    }

    pub fn equals_1(self) -> bool {
        match self {
            Self::Unsigned8(val) => val == 1_u8,
            Self::Unsigned16(val) => val == 1_u16,
            Self::Unsigned32(val) => val == 1_u32,
            Self::Unsigned64(val) => val == 1_u64,
            Self::Unsigned128(val) => val == 1_u128,
            Self::UnsignedBig(val) => val == ubig!(1),
            Self::Signed8(val) => val == 1_i8,
            Self::Signed16(val) => val == 1_i16,
            Self::Signed32(val) => val == 1_i32,
            Self::Signed64(val) => val == 1_i64,
            Self::Signed128(val) => val == 1_i128,
            Self::SignedBig(val) => val == ibig!(1),
            _ => false,
        }
    }
}

impl Encode for VmTerm {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        match self {
            Self::Hash160(ref v) => {
                bincode::Encode::encode(&0x00_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Hash256(ref v) => {
                bincode::Encode::encode(&0x01_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Hash512(ref v) => {
                bincode::Encode::encode(&0x02_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Unsigned8(ref v) => {
                bincode::Encode::encode(&0x03_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Unsigned16(ref v) => {
                bincode::Encode::encode(&0x04_u8, encoder)?;
                let v: [u8; 2] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Unsigned32(ref v) => {
                bincode::Encode::encode(&0x05_u8, encoder)?;
                let v: [u8; 4] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Unsigned64(ref v) => {
                bincode::Encode::encode(&0x06_u8, encoder)?;
                let v: [u8; 8] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Unsigned128(ref v) => {
                bincode::Encode::encode(&0x07_u8, encoder)?;
                let v: [u8; 16] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::UnsignedBig(ref v) => {
                let bytes = v.to_le_bytes();
                bincode::Encode::encode(&0x08_u8, encoder)?;
                bincode::Encode::encode(&bytes, encoder)?;
            }

            Self::Signed8(ref v) => {
                bincode::Encode::encode(&0x09_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Signed16(ref v) => {
                bincode::Encode::encode(&0x0a_u8, encoder)?;
                let v: [u8; 2] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Signed32(ref v) => {
                bincode::Encode::encode(&0x0b_u8, encoder)?;
                let v: [u8; 4] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Signed64(ref v) => {
                bincode::Encode::encode(&0x0c_u8, encoder)?;
                let v: [u8; 8] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::Signed128(ref v) => {
                bincode::Encode::encode(&0x0d_u8, encoder)?;
                let v: [u8; 16] = v.to_le_bytes();
                bincode::Encode::encode(&v, encoder)?;
            }

            Self::SignedBig(ref v) => {
                let sign = v.signum().to_f32() as i8;
                let v = v.abs();

                match sign {
                    -1 => {
                        bincode::Encode::encode(&0x0e_u8, encoder)?;
                        let v: UBig = v.try_into().unwrap();
                        let bytes = v.to_le_bytes();
                        bincode::Encode::encode(&bytes, encoder)?;
                    }

                    1 => {
                        bincode::Encode::encode(&0x0f_u8, encoder)?;
                        let v: UBig = v.try_into().unwrap();
                        let bytes = v.to_le_bytes();
                        bincode::Encode::encode(&bytes, encoder)?;
                    }

                    0 => {
                        bincode::Encode::encode(&0x10_u8, encoder)?;
                    }

                    _ => unreachable!(),
                }
            }

            Self::Hash160Array(ref v) => {
                bincode::Encode::encode(&0x13_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Hash256Array(ref v) => {
                bincode::Encode::encode(&0x14_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Hash512Array(ref v) => {
                bincode::Encode::encode(&0x15_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Unsigned8Array(ref v) => {
                bincode::Encode::encode(&0x16_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Unsigned16Array(ref v) => {
                bincode::Encode::encode(&0x17_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 2);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Unsigned32Array(ref v) => {
                bincode::Encode::encode(&0x18_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 4);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Unsigned64Array(ref v) => {
                bincode::Encode::encode(&0x19_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 8);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Unsigned128Array(ref v) => {
                bincode::Encode::encode(&0x1a_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 16);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::UnsignedBigArray(ref v) => {
                bincode::Encode::encode(&0x1b_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 32);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Signed8Array(ref v) => {
                bincode::Encode::encode(&0x1c_u8, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }

            Self::Signed16Array(ref v) => {
                bincode::Encode::encode(&0x1d_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 2);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Signed32Array(ref v) => {
                bincode::Encode::encode(&0x1e_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 4);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Signed64Array(ref v) => {
                bincode::Encode::encode(&0x1f_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 8);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::Signed128Array(ref v) => {
                bincode::Encode::encode(&0x20_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * 16);
                for v in v.iter() {
                    buf.extend_from_slice(&v.to_le_bytes());
                }
                bincode::Encode::encode(&buf, encoder)?;
            }

            Self::SignedBigArray(ref v) => {
                bincode::Encode::encode(&0x21_u8, encoder)?;
                let mut buf = Vec::with_capacity(v.len() * (32 + 1));
                for v in v.iter() {
                    let sign = v.signum().to_f32() as i8;
                    let v = v.abs();

                    match sign {
                        -1 => {
                            buf.extend_from_slice(&[0x0e_u8]);
                            let v: UBig = v.try_into().unwrap();
                            let bytes = v.to_le_bytes();
                            buf.extend_from_slice(&bytes);
                        }

                        1 => {
                            buf.extend_from_slice(&[0x0f_u8]);
                            let v: UBig = v.try_into().unwrap();
                            let bytes = v.to_le_bytes();
                            buf.extend_from_slice(&bytes);
                        }

                        0 => {
                            buf.extend_from_slice(&[0x10_u8]);
                        }

                        _ => unreachable!(),
                    }
                }
                bincode::Encode::encode(&buf, encoder)?;
            }
        }

        Ok(())
    }
}

impl Decode for VmTerm {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let t: u8 = bincode::Decode::decode(decoder)?;

        match t {
            0x00 => {
                let v: [u8; 20] = bincode::Decode::decode(decoder)?;
                Ok(VmTerm::Hash160(v))
            }

            0x01 => {
                let v: [u8; 32] = bincode::Decode::decode(decoder)?;
                Ok(VmTerm::Hash256(v))
            }

            0x02 => {
                let v: [u8; 64] = bincode::Decode::decode(decoder)?;
                Ok(VmTerm::Hash512(v))
            }

            0x03 => {
                let v: u8 = bincode::Decode::decode(decoder)?;
                Ok(VmTerm::Unsigned8(v))
            }

            0x04 => {
                let v: [u8; 2] = bincode::Decode::decode(decoder)?;
                let v = u16::from_le_bytes(v);
                Ok(VmTerm::Unsigned16(v))
            }

            0x05 => {
                let v: [u8; 4] = bincode::Decode::decode(decoder)?;
                let v = u32::from_le_bytes(v);
                Ok(VmTerm::Unsigned32(v))
            }

            0x06 => {
                let v: [u8; 8] = bincode::Decode::decode(decoder)?;
                let v = u64::from_le_bytes(v);
                Ok(VmTerm::Unsigned64(v))
            }

            0x07 => {
                let v: [u8; 16] = bincode::Decode::decode(decoder)?;
                let v = u128::from_le_bytes(v);
                Ok(VmTerm::Unsigned128(v))
            }

            0x08 => {
                let v: Vec<u8> = bincode::Decode::decode(decoder)?;
                let v = UBig::from_le_bytes(&v);
                Ok(VmTerm::UnsignedBig(v))
            }

            0x09 => {
                let v: i8 = bincode::Decode::decode(decoder)?;
                Ok(VmTerm::Signed8(v))
            }

            0x0a => {
                let v: [u8; 2] = bincode::Decode::decode(decoder)?;
                let v = i16::from_le_bytes(v);
                Ok(VmTerm::Signed16(v))
            }

            0x0b => {
                let v: [u8; 4] = bincode::Decode::decode(decoder)?;
                let v = i32::from_le_bytes(v);
                Ok(VmTerm::Signed32(v))
            }

            0x0c => {
                let v: [u8; 8] = bincode::Decode::decode(decoder)?;
                let v = i64::from_le_bytes(v);
                Ok(VmTerm::Signed64(v))
            }

            0x0d => {
                let v: [u8; 16] = bincode::Decode::decode(decoder)?;
                let v = i128::from_le_bytes(v);
                Ok(VmTerm::Signed128(v))
            }

            0x0e => {
                let v: Vec<u8> = bincode::Decode::decode(decoder)?;
                let u = UBig::from_le_bytes(&v);
                let i: IBig = u.try_into().map_err(|_| {
                    bincode::error::DecodeError::OtherString("could not parse i256".to_owned())
                })?;
                Ok(VmTerm::SignedBig(i * ibig!(-1)))
            }

            0x0f => {
                let v: Vec<u8> = bincode::Decode::decode(decoder)?;
                let u = UBig::from_le_bytes(&v);
                let i: IBig = u.try_into().map_err(|_| {
                    bincode::error::DecodeError::OtherString("could not parse i256".to_owned())
                })?;
                Ok(VmTerm::SignedBig(i))
            }

            0x10 => Ok(VmTerm::SignedBig(IBig::zero())),

            _ => Err(bincode::error::DecodeError::OtherString(
                "invalid term type".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::{ibig, ubig};

    #[test]
    fn test_hash160_encode_decode() {
        let t = VmTerm::Hash160([0xff; 20]);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x00]);
        assert_eq!(encoded.len(), 21);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_hash256_encode_decode() {
        let t = VmTerm::Hash256([0xff; 32]);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x01]);
        assert_eq!(encoded.len(), 33);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_hash512_encode_decode() {
        let t = VmTerm::Hash512([0xff; 64]);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x02]);
        assert_eq!(encoded.len(), 65);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u8_encode_decode() {
        let t = VmTerm::Unsigned8(12);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x03]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u16_encode_decode() {
        let t = VmTerm::Unsigned16(12324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x04]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u32_encode_decode() {
        let t = VmTerm::Unsigned32(1254324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x05]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u64_encode_decode() {
        let t = VmTerm::Unsigned64(143254324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x06]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u128_encode_decode() {
        let t = VmTerm::Unsigned128(143254354354324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x07]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_u256_encode_decode() {
        let i = ubig!(42432);
        let t = VmTerm::UnsignedBig(i);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x08]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i8_encode_decode_negative() {
        let t = VmTerm::Signed8(-14);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x09]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i8_encode_decode_positive() {
        let t = VmTerm::Signed8(12);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x09]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i16_encode_decode_negative() {
        let t = VmTerm::Signed16(-14423);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0a]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i16_encode_decode_positive() {
        let t = VmTerm::Signed16(12324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0a]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i32_encode_decode_negative() {
        let t = VmTerm::Signed32(-1432543423);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0b]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i32_encode_decode_positive() {
        let t = VmTerm::Signed32(1254324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0b]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i64_encode_decode_negative() {
        let t = VmTerm::Signed64(-143254423423);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0c]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i64_encode_decode_positive() {
        let t = VmTerm::Signed64(143254324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0c]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i128_encode_decode_negative() {
        let t = VmTerm::Signed128(-143254432432423423423);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0d]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i128_encode_decode_positive() {
        let t = VmTerm::Signed128(143254324324);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0d]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i256_encode_decode_negative() {
        let i = ibig!(-42432);
        let t = VmTerm::SignedBig(i);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0e]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i256_encode_decode_positive() {
        let i = ibig!(42432);
        let t = VmTerm::SignedBig(i);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded[..1], &[0x0f]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_i256_encode_decode_zero() {
        let i = ibig!(0);
        let t = VmTerm::SignedBig(i);
        let encoded = crate::codec::encode_to_vec(&t).unwrap();
        assert_eq!(&encoded, &[0x10]);
        assert_eq!(crate::codec::decode::<VmTerm>(&encoded).unwrap(), t);
    }

    #[test]
    fn test_equals_1() {
        assert_eq!(VmTerm::Unsigned8(1).equals_1(), true);
        assert_eq!(VmTerm::Unsigned16(1).equals_1(), true);
        assert_eq!(VmTerm::Unsigned32(1).equals_1(), true);
        assert_eq!(VmTerm::Unsigned64(1).equals_1(), true);
        assert_eq!(VmTerm::Unsigned128(1).equals_1(), true);
        assert_eq!(VmTerm::UnsignedBig(ubig!(1)).equals_1(), true);
        assert_eq!(VmTerm::Signed8(1).equals_1(), true);
        assert_eq!(VmTerm::Signed16(1).equals_1(), true);
        assert_eq!(VmTerm::Signed32(1).equals_1(), true);
        assert_eq!(VmTerm::Signed64(1).equals_1(), true);
        assert_eq!(VmTerm::Signed128(1).equals_1(), true);
        assert_eq!(VmTerm::SignedBig(ibig!(1)).equals_1(), true);

        assert_eq!(VmTerm::Unsigned8(10).equals_1(), false);
        assert_eq!(VmTerm::Unsigned16(10).equals_1(), false);
        assert_eq!(VmTerm::Unsigned32(10).equals_1(), false);
        assert_eq!(VmTerm::Unsigned64(10).equals_1(), false);
        assert_eq!(VmTerm::Unsigned128(10).equals_1(), false);
        assert_eq!(VmTerm::UnsignedBig(ubig!(10)).equals_1(), false);
        assert_eq!(VmTerm::Signed8(10).equals_1(), false);
        assert_eq!(VmTerm::Signed16(10).equals_1(), false);
        assert_eq!(VmTerm::Signed32(10).equals_1(), false);
        assert_eq!(VmTerm::Signed64(10).equals_1(), false);
        assert_eq!(VmTerm::Signed128(10).equals_1(), false);
        assert_eq!(VmTerm::SignedBig(ibig!(10)).equals_1(), false);
    }
}
