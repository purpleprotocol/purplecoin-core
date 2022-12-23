// Copyright (c) 2022 Octavian Oncescu
// Copyright (c) 2022 The Purplecoin Core developers
// Licensed under the Apache License, Version 2.0 see LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license, see
// LICENSE-MIT or http://opensource.org/licenses/MIT

use bincode::{Decode, Encode};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum OP {
    Func = 0x00,
    ChainId = 0x07,
    ChainHeight = 0x08,
    ChainTimestamp = 0x09,
    IsCoinbase = 0x0a,
    PrevBlockHash = 0x0b,
    NSequence = 0x0c,
    RandomHash160Var = 0x0d,
    RandomHash256Var = 0x0e,
    RandomHash512Var = 0x0f,
    RandomUnsigned8 = 0x10,
    RandomUnsigned16 = 0x11,
    RandomUnsigned32 = 0x12,
    RandomUnsigned64 = 0x13,
    RandomUnsigned128 = 0x14,
    RandomSigned8 = 0x15,
    RandomSigned16 = 0x16,
    RandomSigned32 = 0x17,
    RandomSigned64 = 0x18,
    RandomSigned128 = 0x19,
    Hash160Var = 0x20,
    Hash256Var = 0x21,
    Hash512Var = 0x22,
    Unsigned8Var = 0x23,
    Unsigned16Var = 0x24,
    Unsigned32Var = 0x25,
    Unsigned64Var = 0x26,
    Unsigned128Var = 0x27,
    UnsignedBigVar = 0x28,
    Signed8Var = 0x29,
    Signed16Var = 0x2a,
    Signed32Var = 0x2b,
    Signed64Var = 0x2c,
    Signed128Var = 0x2d,
    SignedBigVar = 0x2e,
    Hash160ArrayVar = 0x31,
    Hash256ArrayVar = 0x32,
    Hash512ArrayVar = 0x33,
    Unsigned8ArrayVar = 0x34,
    Unsigned16ArrayVar = 0x35,
    Unsigned32ArrayVar = 0x36,
    Unsigned64ArrayVar = 0x37,
    Unsigned128ArrayVar = 0x38,
    UnsignedBigArrayVar = 0x39,
    Signed8ArrayVar = 0x3a,
    Signed16ArrayVar = 0x3b,
    Signed32ArrayVar = 0x3c,
    Signed64ArrayVar = 0x3d,
    Signed128ArrayVar = 0x3e,
    SignedBigArrayVar = 0x3f,
    ArrayLen = 0x4a,
    GetType = 0x4b,
    IsNaN = 0x4c,
    IsInf = 0x4d,
    IsMinusInf = 0x4e,
    ClearStack = 0x4f,
    Add = 0x50,
    Sub = 0x51,
    Mult = 0x52,
    Div = 0x53,
    BitSHLeft = 0x54,
    BitSHRight = 0x55,
    BitXOR = 0x56,
    Loop = 0x57,
    Break = 0x58,
    BreakIf = 0x59,
    BreakIfn = 0x5a,
    BreakIfEq = 0x5b,
    BreakIfNeq = 0x5c,
    BreakIfLeq = 0x5d,
    BreakIfGeq = 0x5e,
    BreakIfLt = 0x5f,
    BreakIfGt = 0x60,
    Continue = 0x61,
    ContinueIf = 0x62,
    ContinueIfn = 0x63,
    ContinueIfEq = 0x64,
    ContinueIfNeq = 0x65,
    ContinueIfLeq = 0x66,
    ContinueIfGeq = 0x67,
    ContinueIfLt = 0x68,
    ContinueIfGt = 0x69,
    Depth = 0x6a,
    IfDup = 0x6b,
    Drop = 0x6c,
    Dup = 0x6d,
    Nip = 0x6e,
    Over = 0x6f,
    Pick = 0x70,
    Roll = 0x71,
    Rot = 0x72,
    Swap = 0x73,
    Tuck = 0x74,
    Drop2 = 0x75,
    Dup2 = 0x76,
    Dup3 = 0x77,
    Over2 = 0x78,
    Rot2 = 0x79,
    Swap2 = 0x7a,
    Size = 0x7b,
    Substr = 0x7c,
    Left = 0x7d,
    Right = 0x7e,
    BitInvert = 0x7f,
    IsUTF8 = 0x80,
    Add1 = 0x82,
    Sub1 = 0x83,
    Min = 0x84,
    Max = 0x85,
    Within = 0x86,
    BoolAnd = 0x87,
    BoolOr = 0x88,
    Negate = 0x89,
    Abs = 0x8a,
    PushPrevScriptOuts = 0x8b,
    PushPrevScriptOutsLen = 0x8c,
    PushPrevScriptOut = 0x8d,
    PushExecCount = 0x8e,
    FlushToScriptOuts = 0x8f,
    PopToScriptOuts = 0x90,
    PickToScriptOuts = 0x91,
    ToHex = 0x9a,
    FromHex = 0x9b,
    Call = 0xaf,
    Concat = 0xb0,
    Eq = 0xb1,
    Neq = 0xb2,
    If = 0xb3,
    Ifn = 0xb4,
    Else = 0xb5,
    End = 0xb6,
    Verify = 0xb7,
    ReturnFunc = 0xb8,
    Return = 0xb9,
    EqVerify = 0xba,
    Lt = 0xbb,
    Gt = 0xbc,
    Leq = 0xbd,
    Geq = 0xbe,
    IfLt = 0xbf,
    IfGt = 0xc0,
    IfLeq = 0xc1,
    IfGeq = 0xc2,
    IfEq = 0xc3,
    IfNeq = 0xc4,
    LtVerify = 0xc5,
    GtVerify = 0xc6,
    LeqVerify = 0xc7,
    GeqVerify = 0xc8,
    NeqVerify = 0xc9,
    CastTo = 0xca,
    PushOut = 0xcf,
    PushOutVerify = 0xe0,
    PushCoinbaseOut = 0xe1,
    PushColouredCoinbaseOut = 0xe2,
    PushOutIf = 0xe3,
    PushOutIfEq = 0xe4,
    PushOutIfNeq = 0xe5,
    PushOutIfLt = 0xe6,
    PushOutIfGt = 0xe7,
    PushOutIfLeq = 0xe8,
    PushOutIfGeq = 0xe9,
    HasInput = 0xea,
    HasOutput = 0xeb,
    HasColouredInput = 0xec,
    HasColouredOutput = 0xed,
    Sha256 = 0xee,
    Sha512 = 0xef,
    Keccak256 = 0xf0,
    Keccak512 = 0xf1,
    Blake2b256 = 0xf2,
    Blake2b512 = 0xf3,
    Blake3_160 = 0xf4,
    Blake3_256 = 0xf5,
    Blake3_512 = 0xf6,
    Blake3_256_160 = 0xf7,
    Blake3_256Keyed = 0xf8,
    Blake3_512Keyed = 0xf9,
    Blake3_160Keyed = 0xfa,
    Blake3_256Internal = 0xfb,
    Blake3_512Internal = 0xfc,
    Blake3_256_160Internal = 0xfd,
    Ripemd160 = 0xfe,
    Nop = 0xff,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_opcode_to_byte() {
        let encoded = crate::codec::encode_to_vec(&OP::Func).unwrap();
        assert_eq!(encoded.as_slice(), &[0x00]);
        let encoded = crate::codec::encode_to_vec(&OP::PushCoinbaseOut).unwrap();
        assert_eq!(encoded.as_slice(), &[0xe1]);
        let encoded = crate::codec::encode_to_vec(&OP::Nop).unwrap();
        assert_eq!(encoded.as_slice(), &[0xff]);
    }
}
