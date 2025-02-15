/// The opcode of the `PUSH[n]` instruction, given a byte count `n`.
pub(crate) fn get_push_opcode(n: u8) -> u8 {
    assert!(n > 0);
    assert!(n <= 32);
    0x60 + (n as u8 - 1)
}

/// The opcode of a standard instruction (not a `PUSH`).
pub(crate) fn get_opcode(mnemonic: &str) -> u8 {
    match mnemonic.to_uppercase().as_str() {
        "STOP" => 0x00,
        "ADD" => 0x01,
        "MUL" => 0x02,
        "SUB" => 0x03,
        "DIV" => 0x04,
        "SDIV" => 0x05,
        "MOD" => 0x06,
        "SMOD" => 0x07,
        "ADDMOD" => 0x08,
        "MULMOD" => 0x09,
        "EXP" => 0x0a,
        "SIGNEXTEND" => 0x0b,
        "LT" => 0x10,
        "GT" => 0x11,
        "SLT" => 0x12,
        "SGT" => 0x13,
        "EQ" => 0x14,
        "ISZERO" => 0x15,
        "AND" => 0x16,
        "OR" => 0x17,
        "XOR" => 0x18,
        "NOT" => 0x19,
        "BYTE" => 0x1a,
        "SHL" => 0x1b,
        "SHR" => 0x1c,
        "SAR" => 0x1d,
        "KECCAK256" => 0x20,
        "ADDRESS" => 0x30,
        "BALANCE" => 0x31,
        "ORIGIN" => 0x32,
        "CALLER" => 0x33,
        "CALLVALUE" => 0x34,
        "CALLDATALOAD" => 0x35,
        "CALLDATASIZE" => 0x36,
        "CALLDATACOPY" => 0x37,
        "CODESIZE" => 0x38,
        "CODECOPY" => 0x39,
        "GASPRICE" => 0x3a,
        "EXTCODESIZE" => 0x3b,
        "EXTCODECOPY" => 0x3c,
        "RETURNDATASIZE" => 0x3d,
        "RETURNDATACOPY" => 0x3e,
        "EXTCODEHASH" => 0x3f,
        "BLOCKHASH" => 0x40,
        "COINBASE" => 0x41,
        "TIMESTAMP" => 0x42,
        "NUMBER" => 0x43,
        "DIFFICULTY" => 0x44,
        "GASLIMIT" => 0x45,
        "CHAINID" => 0x46,
        "BASEFEE" => 0x48,
        "PROVER_INPUT" => 0x49,
        "POP" => 0x50,
        "MLOAD" => 0x51,
        "MSTORE" => 0x52,
        "MSTORE8" => 0x53,
        "SLOAD" => 0x54,
        "SSTORE" => 0x55,
        "JUMP" => 0x56,
        "JUMPI" => 0x57,
        "GETPC" => 0x58,
        "MSIZE" => 0x59,
        "GAS" => 0x5a,
        "JUMPDEST" => 0x5b,
        "GET_STATE_ROOT" => 0x5c,
        "SET_STATE_ROOT" => 0x5d,
        "GET_RECEIPT_ROOT" => 0x5e,
        "SET_RECEIPT_ROOT" => 0x5f,
        "DUP1" => 0x80,
        "DUP2" => 0x81,
        "DUP3" => 0x82,
        "DUP4" => 0x83,
        "DUP5" => 0x84,
        "DUP6" => 0x85,
        "DUP7" => 0x86,
        "DUP8" => 0x87,
        "DUP9" => 0x88,
        "DUP10" => 0x89,
        "DUP11" => 0x8a,
        "DUP12" => 0x8b,
        "DUP13" => 0x8c,
        "DUP14" => 0x8d,
        "DUP15" => 0x8e,
        "DUP16" => 0x8f,
        "SWAP1" => 0x90,
        "SWAP2" => 0x91,
        "SWAP3" => 0x92,
        "SWAP4" => 0x93,
        "SWAP5" => 0x94,
        "SWAP6" => 0x95,
        "SWAP7" => 0x96,
        "SWAP8" => 0x97,
        "SWAP9" => 0x98,
        "SWAP10" => 0x99,
        "SWAP11" => 0x9a,
        "SWAP12" => 0x9b,
        "SWAP13" => 0x9c,
        "SWAP14" => 0x9d,
        "SWAP15" => 0x9e,
        "SWAP16" => 0x9f,
        "LOG0" => 0xa0,
        "LOG1" => 0xa1,
        "LOG2" => 0xa2,
        "LOG3" => 0xa3,
        "LOG4" => 0xa4,
        "PANIC" => 0xa5,
        "CREATE" => 0xf0,
        "CALL" => 0xf1,
        "CALLCODE" => 0xf2,
        "RETURN" => 0xf3,
        "DELEGATECALL" => 0xf4,
        "CREATE2" => 0xf5,
        "GET_CONTEXT" => 0xf6,
        "SET_CONTEXT" => 0xf7,
        "CONSUME_GAS" => 0xf8,
        "EXIT_KERNEL" => 0xf9,
        "STATICCALL" => 0xfa,
        "MLOAD_GENERAL" => 0xfb,
        "MSTORE_GENERAL" => 0xfc,
        "REVERT" => 0xfd,
        "INVALID" => 0xfe,
        "SELFDESTRUCT" => 0xff,
        _ => panic!("Unrecognized mnemonic {}", mnemonic),
    }
}
